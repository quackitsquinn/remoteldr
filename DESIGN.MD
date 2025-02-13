# RemoteLDR Design

## The Problem

I do most of my development on my MacBook Pro, but I want to write a educational piece of malware that runs on Windows. I could use a VM, but that is a pain to set up and is incredibly slow on my machine due to it being an M3 processor. I do have another laptop though. It's an older machine and I don't want to use it for development, but I could use it as a remote machine to run my binaries on. 

I have not done much research on prior art, but the little I have done seem to be mostly focused on remote development and sysadmin tasks. This tool is supposed to be incredibly easy to set up and "just work™️".

## Overview

I'm thinking of protobufs and TCP sockets for the communication between the client and server.

I'm tempted to use gRPC because of it's simplicity, but since this is in rust it would be nice to avoid the tokio dependency (tokio is good, but the giant dependency tree is a bit annoying). 

I will probably do some testing if including tokio would greatly increase execution time

I am not going to bother to write out the protobufs here. At this point I know that there will be 2 core messages: `Command` and `Response`. This is to keep the communication simple between the client and server.


## Keeping connections alive

I have though of two approaches for this. 

1. A daemon process that will stay alive until the user calls `remoteldr client disconnect`. 
    - This won't make accessing the shell more difficult, and would be a cleaner development experience.
    - The communication between the client and daemon process would have to be done through a socket, but this could just be essentially a pseudo proxy that just passes all messages to the server.
        - Im not concerned about the overhead of this, because localhost connections are near instantaneous.
        - If I go the gRPC route, this might be a bit more difficult.
            - This would probably be fine though, you could just repetitively read from the socket and write to the server.
    - Down the road the daemon process could manage multiple connections to the server, but that is not a priority right now, as the use case for this seems a bit niche.
    - The daemon process would also invalidate the connection if a certain amount of time has passed since the last message was sent.
        - This would be a good feature to add later on, but is not necessary right now.

2. An interactive shell that will keep the connection alive until exits.
    - This would be a bit more difficult to implement, but would be more user friendly.
    - User friendliness is not equal to a better development experience.
    - This would instantly allow more than one connection at the same time, but again, the use case seems a bit niche.
    - This could be a good feature to add later on, but I don't think it is necessary right now.

## Configuration

It might be worthwhile to add a config file in the same vein of `Cargo.toml`. This would allow the user to specify the server address and port, and other settings. This would be additionally useful for rust, because down the road I am planning on adding a cargo plugin that will allow the user to run binaries on a remote machine.

## Future Cargo Plugin

If this gets a cargo plugin, the plugin could automatically determine the server address and port from the config file, load the binary onto the server, and run it. This would be a very useful feature for remote development, and would be a good way to get people to use this tool. This would also be able to automatically determine build targets, which would be incredibly useful for cross platform development.

`cargo remoteldr run` is better than `cargo build --target x86_64-windows-gnu && remoteldr client run --binary target/x86_64-windows-gnu/debug/binary`

## Usage

The server interface will probably be pretty simple.

```bash

remoteldr server --port 8080

# You could also specify a folder to store transferred data in.

remoteldr server --port 8080 --data-path /path/to/folder

# You could also specify a name for the server. The default name is the hostname of the machine.
remoteldr server --port 8080 --name "Server Name"

```

However, the client interface will be a bit more complex.

```bash

# Creates a daemon process that will run until remoteldr client disconnect is called.
remoteldr client start --host ip --port 8080

# Send a file to the server. This is different from sending a binary to run.
remoteldr client send /local/path /remote/path

# Send a executable to the server to run and run it.
# --wait-for-exit will make the client wait for the server to send an exit code before returning.
remoteldr client run /path/to/binary

# Send a executable to the server to run and run it with arguments.
remoteldr client run /path/to/binary --args "arg1 arg2 arg3"

# Kill the running process on the server.
remoteldr client kill

# Get the exit code of the last process that was run.
remoteldr client get_exit_code

# Disconnect from the server.
remoteldr client disconnect

```

