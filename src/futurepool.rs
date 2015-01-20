use std::sync::TaskPool;
use std::sync::Future;
use std::sync::mpsc::channel;

struct FutureTube<A,B> {
    fp: FuturePool,
    fut: Result<Future<A>, B>,
}
impl<A,B> FutureTube<A,B> {
    fn map<F,C>(&self, blk: F) -> FutureTube<C, B>
        where F : FnOnce(A) -> C, F : Send, C : Send, B : Send, A : Send
    {
        match self.fut {
            Ok(fa) => {
                FutureTube {
                    fp: self.fp,
                    fut: Ok(self.fp.execute(move|| {
                        blk(fa.get())
                    })),
                }
            },
            Err(e) => {
                FutureTube {
                    fp: self.fp,
                    fut: Err(e),
                }
            },
        }
    }
}

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

#[test]
fn test_compose() {
    let fp = FuturePool::new(4);
    for i in range(1,10) {
        fp.execute(|&:| -> u64 {
            i * 2
        }).map(|&: rep: u64| {
            println!("in map, dealing with response: {}", rep);
        }).on_failure(|&: error: &str| {
            println!("failed to process OR respond: {}", error);
        });
    }
}
