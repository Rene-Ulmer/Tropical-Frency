pub struct RefCell<T> {
    val: T,
}

impl<T> RefCell<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }

    pub fn borrow(&self) -> &T {
        &self.val
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *(&self.val as *const T as *mut T) }
    }
}
