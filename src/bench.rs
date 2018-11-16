extern crate test;

use self::test::Bencher;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

#[bench]
fn bench_std_thread(b: &mut Bencher) {

    let num = Arc::new(AtomicUsize::new(0));

    b.iter(|| {

        let num = num.clone();

        ::std::thread::spawn(move || {
            num.fetch_add(1, Ordering::Relaxed);
        }).join().unwrap();
    });

    println!("{:#?}", num);
}

#[bench]
fn bench_borrowed_thread(b: &mut Bencher) {

    let num = Arc::new(AtomicUsize::new(0));

    b.iter(|| {
        let num = num.clone();

        super::spawn(|| {
            num.fetch_add(1, Ordering::Relaxed);
        }).join().unwrap();
    });

    println!("{:#?}", num);
}