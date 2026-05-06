use bencher::{Bencher, benchmark_group, benchmark_main};
use fableflake::{Fableflake, decompose};
use std::thread;

fn bench_new(b: &mut Bencher) {
    b.iter(Fableflake::new);
}

fn bench_next_id(b: &mut Bencher) {
    let sf = Fableflake::new().expect("Could not create Fableflake");
    b.iter(|| sf.next_id());
}

fn bench_decompose(b: &mut Bencher) {
    let sf = Fableflake::new().expect("Could not create Fableflake");
    let next_id = sf.next_id().expect("Could not get next id");

    b.iter(|| decompose(next_id));
}

fn bench_parallel_next_id(b: &mut Bencher) {
    let sf = Fableflake::new().expect("Could not create Fableflake");
    b.iter(|| {
        let mut workers = Vec::new();
        for _ in 0..4 {
            let thread_sf = sf.clone();
            workers.push(thread::spawn(move || {
                for _ in 0..500 {
                    let _ = thread_sf.next_id().expect("Could not get next id");
                }
            }));
        }

        for worker in workers {
            worker.join().expect("worker panicked");
        }
    });
}

benchmark_group!(
    fableflake_perf,
    bench_new,
    bench_next_id,
    bench_decompose,
    bench_parallel_next_id
);

benchmark_main!(fableflake_perf);
