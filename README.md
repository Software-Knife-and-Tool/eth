## Eth

An *iced*-rs UI for the *thorn*[0] programming environment, under heavy development. Not wholly functional.

An experiment in building simple control-based UIs with *Thorn* scripting.

###### Building *eth*

------

You can build *eth* through the usual mechanism(s).

```
cargo build
```

The included makefile has some convenience functions

```
make help
```

will tell you about them. A more complete build command:

```
make commit
```

runs the formatter and linters.

###### Running *eth*

------

Create a `~/.config/eth` directory and do a `make config`. Sample configuration files are included in the source distribution.

[0] - https://github.com/Software-Knife-and-Tool/thorn
