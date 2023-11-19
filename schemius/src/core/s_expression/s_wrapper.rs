use core::fmt::Debug;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Formatter,
    rc::Rc,
};

pub struct SWrapper<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> SWrapper<T> {
    pub fn new(value: T) -> Self {
        Self { inner: Rc::new(RefCell::new(value)) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    pub fn replace(&self, src: T) -> T {
        std::mem::replace(&mut *self.inner.borrow_mut(), src)
    }
}

impl<T: Debug> Debug for SWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SWrapper {{ inner: {:?} }}", self.inner)
    }
}

impl<T: Clone> Clone for SWrapper<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
