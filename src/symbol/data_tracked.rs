use DataUnsafe;
use LibUnsafe;
use Symbol;

// A pointer to a function from a shared library which uses ref counting to avoid outliving the library.
#[derive(Clone)]
pub struct DataTracked<T, TLib> {
    data: DataUnsafe<T>,
    _lib: TLib,
}

impl <T, TLib> DataTracked<T, TLib>
    where TLib: AsRef<LibUnsafe> + Clone {
    pub fn new(data: DataUnsafe<T>, lib: TLib) -> Self {
        DataTracked {
            data: data,
            _lib: lib,
        }
    }
}

impl <'a, T, TLib> Symbol<&'a T> for DataTracked<T, TLib> {
    unsafe fn get(&self) -> &'a T {
        self.data.get()
    }
}
