use std::sync::TaskPool;
use std::sync::Future;
use std::sync::mpsc::channel;

pub struct FuturePool {
    pool: TaskPool
}
impl FuturePool {
    pub fn new(threads: usize) -> FuturePool {
        FuturePool{pool: TaskPool::new(threads)}
    }

    pub fn execute<F,A : Send>(&self, blk: F) -> Future<A>
        where F : FnOnce() -> A, F : Send
    {
        let (tx, rx) = channel();
        self.pool.execute(move|| {
            tx.send(blk());
        });
        Future::from_receiver(rx)
    }
}

#[test]
fn test_basic() {
    let fp = FuturePool::new(4);
    let mut rep = fp.execute(move|| {
        "result"
    });
    assert!(rep.get() == "result");
}
