This is a system that makes the server able to handle multiple client connections concurrently.

It has a *fixed* number of *worker threads* and a *queue of jobs*.
Each worker waits for a job in the queue, picks one, executes it, and then goes to the next job. 
That allows for concurrent client connection because 


### Implementation

```rust_http_server::threadpool::ThreadPool```
The Threadpool is the main class, and it is a struct that holds the *Worker* class and *mpsc::Sender*.

The *Worker* class is the object that *receives* jobs and executes them on a *new thread*.
The *mpsc* is an important concept as it is the `multi-producer single-consumer`, basically a channel of data where data can come from many places but can only go to one, and it's *sender* object is what enables sending data through the channel.
In the other side, the *Worker* class has the *receiver* object, however, there only exists one instace of it and it is shared among workers, so its encapsulated in a Arc<Mutex<>>.
The idea is that you can have *multiple* senders, but only a *single* receiver - hence, `multi-producer single-consumer`.

**Graceful Shutdown:**

