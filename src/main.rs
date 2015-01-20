extern crate test;
extern crate futurepool;
use test::Bencher;
use std::sync::Future;
use futurepool::FuturePool;

fn test_handle(req: &str) -> &str {
    "here's some cats"
}

#[bench]
fn futurepool_bench(b: &mut Bencher) {
    let fp = FuturePool::new(1);

    b.iter(|| {
        let mut rep = fp.execute(move|| {
            test_handle("get cats")
        });
        rep.get();
    });
}

#[bench]
fn spawn_bench(b: &mut Bencher) {
    b.iter(|| {
        let mut rep = Future::spawn(move|| {
            test_handle("get cats")
        });
        rep.get();
    });
}
