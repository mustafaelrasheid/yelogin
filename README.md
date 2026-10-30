# yelogin
A simple login and authentication program.
It uses yescrypt (and only yescrypt), and it uses `/etc/passwd` and
`/etc/shadow` to process login info. If no argument is provided, login would
run the shell specified; if a path is provided as an argument, it would be run
instead of the shell. The privilege dropped to the UID and GID specified in
passwd.
