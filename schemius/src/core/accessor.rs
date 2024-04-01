use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex},
};

pub trait Accessor<T: Clone> {
    fn new(src: T) -> Self;
    fn borrow(&self) -> impl Deref<Target = T>;
    fn borrow_mut(&self) -> impl DerefMut<Target = T>;
    fn replace(&self, src: T) -> T;
}

#[derive(Clone, Debug)]
pub struct BaseAccessor<T> {
    inner: Rc<RefCell<T>>,
}

#[derive(Clone, Debug)]
pub struct ThreadSafeAccessor<T> {
    inner: Arc<Mutex<T>>,
}

impl<T: Clone> Accessor<T> for BaseAccessor<T> {
    fn new(src: T) -> Self {
        Self { inner: Rc::new(RefCell::new(src)) }
    }
    fn borrow(&self) -> impl Deref<Target = T> {
        self.inner.borrow()
    }
    fn borrow_mut(&self) -> impl DerefMut<Target = T> {
        self.inner.borrow_mut()
    }
    fn replace(&self, src: T) -> T {
        std::mem::replace(&mut *self.inner.borrow_mut(), src)
    }
}

impl<T: Clone> Accessor<T> for ThreadSafeAccessor<T> {
    fn new(src: T) -> Self {
        Self { inner: Arc::new(Mutex::new(src)) }
    }
    fn borrow(&self) -> impl Deref<Target = T> {
        self.inner.lock().unwrap()
    }
    fn borrow_mut(&self) -> impl DerefMut<Target = T> {
        self.inner.lock().unwrap()
    }
    fn replace(&self, src: T) -> T {
        std::mem::replace(&mut *self.inner.lock().unwrap(), src)
    }
}
