[WIP] FuturePool
============
Tiny future pool library.
```rust
extern crate futurepool;
use futurepool::FuturePool;

fn server() {
    let fp = FuturePool::new(std::os::num_cpus() / 4);
    ...
    for req in some_acceptor.iter() {
        // This creates 2 tasks in the future pool
        // to be executed asynchronously:
        //     1. execute the handler
        //     2. respond to the request when #1 is complete
        // If either step fails, any on_failure calls will be
        // executed sequentially.
        fp.execute(|&:| -> Rep {
            user_supplied_handler(req);
        }).map(|&: rep: Rep| {
            req.respond(rep);
        }).on_failure(|&: error: Err<String>| {
            println!("failed to process OR respond: {}", error);
        });
    }
}
```
