# Rust Implementation of thread poll consuming messages from UnixSocket

Goal of this project was to implement simple ThreadPool that will be used
as message handler for messages incoming on UnixSocket.

Basically, idea was that you want to create CLI program that will be triggering
some kind of "actions". Worker would parse messages and spawn processes
to execute them.

In other words we have AsyncProcessing on your local machine.
