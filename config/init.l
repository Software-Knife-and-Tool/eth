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

(mu:intern :eth "load-core"
  (:lambda ()
    (mu:fix
     (:lambda (list)
        (:if (mu:eq list ())
             ()
             ((:lambda ()
                (eth:load (mu:car list))
                (mu:cdr list)))))
   '("/opt/thorn/thorn/core/core.l"
     "/opt/thorn/thorn/core/backquote.l"
     "/opt/thorn/thorn/core/boole.l"
     "/opt/thorn/thorn/core/break.l"
     "/opt/thorn/thorn/core/compile.l"
     "/opt/thorn/thorn/core/ctype.l"
     "/opt/thorn/thorn/core/describe.l"
     "/opt/thorn/thorn/core/environment.l"
     "/opt/thorn/thorn/core/exception.l"
     "/opt/thorn/thorn/core/fasl.l"
     "/opt/thorn/thorn/core/fixnum.l"
     "/opt/thorn/thorn/core/format.l"
     "/opt/thorn/thorn/core/funcall.l"
     "/opt/thorn/thorn/core/function.l"
     "/opt/thorn/thorn/core/lambda.l"
     "/opt/thorn/thorn/core/list.l"
     "/opt/thorn/thorn/core/log.l"
     "/opt/thorn/thorn/core/macro.l"
     "/opt/thorn/thorn/core/map.l"
     "/opt/thorn/thorn/core/namespace.l"
     "/opt/thorn/thorn/core/parse.l"
     "/opt/thorn/thorn/core/read-macro.l"
     "/opt/thorn/thorn/core/read.l"
     "/opt/thorn/thorn/core/repl.l"
     "/opt/thorn/thorn/core/sequence.l"
     "/opt/thorn/thorn/core/sort.l"
     "/opt/thorn/thorn/core/stream.l"
     "/opt/thorn/thorn/core/string.l"
     "/opt/thorn/thorn/core/symbol-macro.l"
     "/opt/thorn/thorn/core/symbol.l"
     "/opt/thorn/thorn/core/time.l"
     "/opt/thorn/thorn/core/type.l"
     "/opt/thorn/thorn/core/vector.l"))))

(eth:load-core)

;;; inspector utilities
(mu:intern :eth "describe-stream" (mu:open :string :output ""))

(mu:intern :eth "describe"
   (:lambda (obj)
     (core:describe obj eth:describe-stream)
     (mu:get-str eth:describe-stream)))

;;; json utilities
(mu:intern :eth "json-cmd-stream" (mu:open :string :output ""))

(mu:intern :eth "write-json"
  (:lambda (form stream)
    (:if (core:stringp form)
         (core:format stream "~S" `(,form))
         (:if (core:vectorp form)
              ((:lambda (len)
                  (core:format stream "[ " ())
                  (mu:fix
                   (:lambda (nth)
                      (:if (mu:eq nth len)
                           nth
                           (:if (mu:fx-lt nth len)
                                ((:lambda ()
                                    (eth:write-json (mu:sv-ref form nth) stream)
                                    (:if (mu:fx-lt nth (core:1- len))
                                         (core:format stream ", " ())                           
                                         ())
                                    (core:1+ nth)))
                                nth)))
                   0)
                  (core:format stream " ]" ()))
               (mu:sv-len form))
              (:if (core:dottedp form)
                   ((:lambda ()
                       (core:format stream "{ ~S : " `(,(mu:car form)))
                       (eth:write-json (mu:cdr form) stream)
                       (core:format stream " }" ())))
                   (core:format stream " ~S " `(,form)))))))

;;; scratchpad functions
(mu:intern :eth "defbutton"
  (:lambda (group label form stream)
    (eth:write-json
       `("defbutton" ,(mu:cons "group" group) ,(mu:cons "label" label) ,(mu:cons "form" form))
     stream)))
