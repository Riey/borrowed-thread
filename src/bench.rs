extern crate test;

use self::test::Bencher;

#[bench]
fn bench_std_thread(b: &mut Bencher) {
    b.iter(|| {
        ::std::thread::spawn(|| {

        }).join().unwrap();
    });
}

#[bench]
fn bench_borrowed_thread(b: &mut Bencher) {
    b.iter(|| {
        super::spawn(|| {

        }).join().unwrap();
    })
}