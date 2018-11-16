#[test]
fn ref_mut_works() {
    let mut owned = "ABC".to_owned();

    let borrowed_handle = super::spawn(|| {
        owned.push('D');
        0
    });

    let ret = borrowed_handle.join().expect("join err");

    assert_eq!(0, ret);
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
