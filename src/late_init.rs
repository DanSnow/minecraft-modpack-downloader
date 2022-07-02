use std::ops::Deref;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct LateInit<T> {
    cell: OnceCell<T>,
}

impl<T> LateInit<T> {
    pub const fn new() -> Self {
        Self { cell: OnceCell::new() }
    }

    pub fn init(&self, value: T) {
        assert!(self.cell.set(value).is_ok());
    }
}

impl<T> Default for LateInit<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.cell.get().unwrap()
    }
}

impl<T, U> AsRef<U> for LateInit<T>
where
    T: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.deref().as_ref()
    }
}
