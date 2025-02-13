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