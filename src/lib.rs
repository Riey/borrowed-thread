#![feature(thread_spawn_unchecked)]
#![feature(test)]

use std::thread;
use std::marker::PhantomData;
use std::io;

struct JoinFlag {
    is_joined: bool,
}

impl JoinFlag {
    fn new() -> Self {
        Self {
            is_joined: false,
        }
    }

    fn set_join(mut self) {
        self.is_joined = true;
    }
}

impl Drop for JoinFlag {
    #[inline]
    fn drop(&mut self) {
        if !self.is_joined {
            panic!("handle must be join before dropped")
        }
    }
}

pub struct BorrowedJoinHandle<'b, T> {
    inner: thread::JoinHandle<T>,
    join_flag: JoinFlag,
    _marker: PhantomData<&'b ()>,
}

impl<'b, T> From<thread::JoinHandle<T>> for BorrowedJoinHandle<'b, T> {

    #[inline]
    fn from(handle: thread::JoinHandle<T>) -> Self {
        Self::new(handle)
    }
}

impl<'b, T> BorrowedJoinHandle<'b, T> {

    #[inline]
    fn new(inner: thread::JoinHandle<T>) -> Self {
        Self {
            inner,
            join_flag: JoinFlag::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn thread(&self) -> &thread::Thread {
        self.inner.thread()
    }

    #[inline]
    pub fn join(self) -> thread::Result<T> {
        self.join_flag.set_join();
        self.inner.join()
    }
}

#[inline]
pub fn spawn_with<'b, F, T>(builder: thread::Builder, f: F) -> io::Result<BorrowedJoinHandle<'b, T>>
    where
        F: FnOnce() -> T,
        F: Send + 'b,
        T: Send + 'b, {
    unsafe {
        match builder.spawn_unchecked(f) {
            Ok(handle) => Ok(handle.into()),
            Err(err) => Err(err),
        }
    }
}

#[inline]
pub fn spawn<'b, F, T>(f: F) -> BorrowedJoinHandle<'b, T>
where
    F: FnOnce() -> T,
    F: Send + 'b,
    T: Send + 'b,
{
    spawn_with(thread::Builder::new(), f).unwrap()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod bench;
