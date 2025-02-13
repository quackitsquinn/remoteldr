# RemoteLDR: A development tool to run binaries on a cross platform remote machine.

## Usage: (WARNING: This is not implemented yet)

### Server:

```bash
remoteldr server --port 8080
```

### Client:

```bash
# Creates a daemon process that will run until remoteldr client disconnect is called.
remoteldr client start --host ip --port 8080

# Commands to run on the remote machine.
remoteldr client run --binary /path/to/binary

# Commands to run on the remote machine with arguments.
remoteldr client run --binary /path/to/binary --args "arg1 arg2 arg3"

remoteldr client kill

remoteldr client disconnect
``` 

## DISCLAIMER:

Whatever you do, do not run this anywhere accessible to the internet. This is not secure in any way, and is not meant to be. This is a development tool, and should be treated as such. It is not my responsibility if you get hacked because you ran this and someone ran xrat or something on your machine.