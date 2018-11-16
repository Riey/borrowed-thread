#[test]
fn ref_mut_works() {
    let mut owned = "ABC".to_owned();

    let borrowed_handle = super::spawn(|| {
        owned.push('D');
        0
    });

    assert_eq!(0, borrowed_handle.join().expect("join err"));
    assert_eq!("ABCD", owned);
}

#[test]
#[should_panic]
fn drop_without_join() {
    let mut owned = "ABC".to_owned();

    let borrowed_handle = super::spawn(|| {
        owned.push('D');
        0
    });

    drop(borrowed_handle);

    assert_eq!("ABCD", owned);
}

#[test]
fn ref_works() {
    let owned = "ABC".to_owned();

    let borrowed_handle = super::spawn(|| {
        assert_eq!("ABC", owned);
        0
    });

    let ret = borrowed_handle.join().expect("join err");

    assert_eq!(0, ret);
}
