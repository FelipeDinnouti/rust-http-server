This is a system that makes the server able to handle multiple client connections concurrently.

It has a *fixed* number of *worker threads* and a *queue of jobs*.
Each worker waits for a job in the queue, picks one, executes it, and then goes to the next job. 
That allows for concurrent client connection because 