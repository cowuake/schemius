use core::fmt::Debug;
use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

pub trait Accessor<T> {
    fn new(src: T) -> Self;
    fn access(&self) -> impl Deref<Target = T>;
    fn access_mut(&self) -> impl DerefMut<Target = T>;
    fn replace(&self, src: T) -> T;
}

#[derive(Clone, Debug)]
pub struct BaseAccessor<T>(Rc<RefCell<T>>);

#[derive(Clone, Debug)]
pub struct ThreadSafeAccessor<T>(Arc<Mutex<T>>);

impl<T> Accessor<T> for BaseAccessor<T> {
    fn new(src: T) -> Self {
        Self(Rc::new(RefCell::new(src)))
    }
    fn access(&self) -> Ref<T> {
        self.0.try_borrow().unwrap()
    }
    fn access_mut(&self) -> RefMut<T> {
        self.0.try_borrow_mut().unwrap()
    }
    fn replace(&self, src: T) -> T {
        std::mem::replace(&mut *self.0.try_borrow_mut().unwrap(), src)
    }
}

impl<T> Accessor<T> for ThreadSafeAccessor<T> {
    fn new(src: T) -> Self {
        Self(Arc::new(Mutex::new(src)))
    }
    fn access(&self) -> MutexGuard<T> {
        self.0.try_lock().unwrap()
    }
    fn access_mut(&self) -> MutexGuard<T> {
        self.0.try_lock().unwrap()
    }
    fn replace(&self, src: T) -> T {
        std::mem::replace(&mut *self.0.try_lock().unwrap(), src)
    }
}
