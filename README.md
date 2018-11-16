# borrowed-thread
thread-safe way to pass borrow to thread::spawn

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

let borrowed_handle = super::spawn(|| {
    owned.push('D');
    0
});

// this will cause panic!
drop(borrowed_handle);

assert_eq!("ABCD", owned);
```


