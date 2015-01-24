# rust-echo

Very basic implemenation of \*NIX `echo` in rust, as a learning exercise.

Commandline args parsing code is basically copy-pasted from getops documentation.

Turns out `echo` a) doesn't read from STDIN, and b) doesn't do any environment variable interpolation
(this is all handled by your shell), so this was very straightforward.
