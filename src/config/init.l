;;; eth namespace
(mu:make-ns :eth)

(mu:intern :eth "load"
  (:lambda (source)
    ((:lambda (ifs)
       (mu:fix
        (:lambda (loop)
          (:if (mu:eof ifs)
               loop
               ((:lambda ()
                  (mu:eval (mu:compile (mu:read ifs :t ())))
                  (mu:eq () loop)))))
       ()))
     (mu:open :file :input source))))

(mu:intern :eth "load-prelude"
  (:lambda ()
    (mu:fix
     (:lambda (list)
        (:if (mu:eq list ())
             ()
             ((:lambda ()
                (eth:load (mu:car list))
                (mu:cdr list)))))
   '("/opt/thorn/thorn/prelude/prelude.l"
     "/opt/thorn/thorn/prelude/backquote.l"
     "/opt/thorn/thorn/prelude/boole.l"
     "/opt/thorn/thorn/prelude/break.l"
     "/opt/thorn/thorn/prelude/compile.l"
     "/opt/thorn/thorn/prelude/ctype.l"
     "/opt/thorn/thorn/prelude/describe.l"
     "/opt/thorn/thorn/prelude/environment.l"
     "/opt/thorn/thorn/prelude/exception.l"
     "/opt/thorn/thorn/prelude/fasl.l"
     "/opt/thorn/thorn/prelude/fixnum.l"
     "/opt/thorn/thorn/prelude/format.l"
     "/opt/thorn/thorn/prelude/funcall.l"
     "/opt/thorn/thorn/prelude/function.l"
     "/opt/thorn/thorn/prelude/inspect.l"
     "/opt/thorn/thorn/prelude/lambda.l"
     "/opt/thorn/thorn/prelude/list.l"
     "/opt/thorn/thorn/prelude/log.l"
     "/opt/thorn/thorn/prelude/macro.l"
     "/opt/thorn/thorn/prelude/map.l"
     "/opt/thorn/thorn/prelude/namespace.l"
     "/opt/thorn/thorn/prelude/parse.l"
     "/opt/thorn/thorn/prelude/read-macro.l"
     "/opt/thorn/thorn/prelude/read.l"
     "/opt/thorn/thorn/prelude/repl.l"
     "/opt/thorn/thorn/prelude/stream.l"
     "/opt/thorn/thorn/prelude/string.l"
     "/opt/thorn/thorn/prelude/symbol-macro.l"
     "/opt/thorn/thorn/prelude/symbol.l"
     "/opt/thorn/thorn/prelude/time.l"
     "/opt/thorn/thorn/prelude/type.l"
     "/opt/thorn/thorn/prelude/vector.l"))))

(eth:load-prelude)
(prelude:%init-ns)

;;; inspector utilities
(mu:intern :eth "inspect-stream" (mu:open :string :output ""))

(mu:intern :eth "inspect"
   (:lambda (obj)
     ((:lambda (inspect)
        (prelude:format
         eth:inspect-stream
         ":type ~A~%:size ~A~%"
         `(,(mu:sv-ref (mu:st-vec inspect) 0)
           ,(mu:sv-ref (mu:st-vec inspect) 1)))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect obj))))

(mu:intern :eth "inspect-tag-keys"
   (:lambda (obj)
     ((:lambda (inspect)
        (prelude:mapc
         (:lambda (tag)
           (prelude:format
            eth:inspect-stream
            "~A;"
            `(,(mu:car tag))))
         (mu:sv-ref (mu:st-vec inspect) 2))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect obj))))

(mu:intern :eth "inspect-tag-values"
   (:lambda (obj)
     ((:lambda (inspect)
        (prelude:mapc
         (:lambda (tag)
           (prelude:format
            eth:inspect-stream
            "~A;"
            `(,(mu:repr :vector (mu:cdr tag)))))
         (mu:sv-ref (mu:st-vec inspect) 2))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect obj))))

(mu:intern :eth "inspect-repr"
   (:lambda (repr)
     ((:lambda (inspect)
        (prelude:format
         eth:inspect-stream
         ":type ~A~%:size ~A~%"
         `(,(mu:sv-ref (mu:st-vec inspect) 0)
           ,(mu:sv-ref (mu:st-vec inspect) 1)))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect (mu:repr :t repr)))))

(mu:intern :eth "inspect-repr-tag-keys"
   (:lambda (repr)
     ((:lambda (inspect)
        (prelude:mapc
         (:lambda (tag)
           (prelude:format
            eth:inspect-stream
            "~A;"
            `(,(mu:car tag))))
         (mu:sv-ref (mu:st-vec inspect) 2))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect (mu:repr :t repr)))))

(mu:intern :eth "inspect-repr-tag-values"
   (:lambda (repr)
     ((:lambda (inspect)
        (prelude:mapc
         (:lambda (tag)
           (prelude:format
            eth:inspect-stream
            "~A;"
            `(,(mu:repr :vector (mu:cdr tag)))))
         (mu:sv-ref (mu:st-vec inspect) 2))
        (mu:get-str eth:inspect-stream))
        (prelude:%inspect (mu:repr :t repr)))))

;;; json utilities
(mu:intern :eth "json-cmd-stream" (mu:open :string :output ""))

(mu:intern :eth "write-json"
  (:lambda (form stream)
    (:if (prelude:stringp form)
         (prelude:format stream "~S" `(,form))
         (:if (prelude:vectorp form)
              ((:lambda (len)
                  (prelude:format stream "[ " ())
                  (mu:fix
                   (:lambda (nth)
                      (:if (mu:eq nth len)
                           nth
                           (:if (mu:fx-lt nth len)
                                ((:lambda ()
                                    (eth:write-json (mu:sv-ref form nth) stream)
                                    (:if (mu:fx-lt nth (prelude:1- len))
                                         (prelude:format stream ", " ())                           
                                         ())
                                    (prelude:1+ nth)))
                                nth)))
                   0)
                  (prelude:format stream " ]" ()))
               (mu:sv-len form))
              (:if (prelude:dottedp form)
                   ((:lambda ()
                       (prelude:format stream "{ ~S : " `(,(mu:car form)))
                       (eth:write-json (mu:cdr form) stream)
                       (prelude:format stream " }" ())))
                   (prelude:format stream " ~S " `(,form)))))))

;;; scratchpad functions
(mu:intern :eth "defbutton"
  (:lambda (group label form stream)
    (eth:write-json
       `("defbutton" ,(mu:cons "group" group) ,(mu:cons "label" label) ,(mu:cons "form" form))
     stream)))
