# borrowed-thread

thread-safe way to pass borrow to thread::spawn

NEED nightly rust!

## benchmark

test bench::bench_borrowed_thread ... bench:      18,902 ns/iter (+/- 690)

test bench::bench_std_thread      ... bench:      18,859 ns/iter (+/- 684)



## examples

### with &mut

```rust
use borrowed_thread;

let mut owned = "ABC".to_owned();

let borrowed_handle = borrowed_thread::spawn(|| {
    owned.push('D');
    0
});

let ret = borrowed_handle.join().expect("join err");

assert_eq!(0, ret);
assert_eq!("ABCD", owned);
```

### with &

```rust
use borrowed_thread;

let owned = "ABC".to_owned();

let borrowed_handle = borrowed_thread::spawn(|| {
    assert_eq!("ABC", owned);
    0
});

let ret = borrowed_handle.join().expect("join err");

assert_eq!(0, ret);
```

### panic when drop without join

```rust
let mut owned = "ABC".to_owned();

let borrowed_handle = borrowed_thread::spawn(|| {
    owned.push('D');
    0
});

// this will cause panic!
drop(borrowed_handle);

assert_eq!("ABCD", owned);
```


