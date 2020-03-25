//! A module related to the Rust standard smart pointer

/// A simple macro to make a new [`Box`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use sugars::boxed;
/// # fn main() {
/// assert_eq!(Box::new(10), boxed!(10));
///
/// // Tuple example
/// let (box_a, box_b) = boxed!(10, "my_str");
/// assert_eq!(Box::new(10), box_a);
/// assert_eq!(Box::new("my_str"), box_b);
/// # }
/// ```
#[macro_export]
macro_rules! boxed {
    ($e:expr) => {
        Box::new($e)
    };
    ($e:expr,) => {
        $crate::boxed!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::boxed!($e)),+,)
    };
}

/// A simple macro to make a new [`Rc`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use std::rc::Rc;
/// use sugars::rc;
/// # fn main() {
/// assert_eq!(Rc::new(10), rc!(10));
/// # }
/// ```
///
/// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
#[macro_export]
macro_rules! rc {
    ($e:expr) => {
        std::rc::Rc::new($e)
    };
    ($e:expr,) => {
        $crate::rc!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::rc!($e)),+,)
    };
}

/// A simple macro to make a new `Cow::Owned` value.
///
/// # Example
/// ```
/// use std::borrow::Cow;
/// use sugars::cow;
/// # fn main() {
/// let expected: Cow<str> = Cow::Owned(String::from("Hello Cow"));
/// let test: Cow<str> = cow!(String::from("Hello Cow"));
///
/// assert_eq!(expected, test);
/// # }
/// ```
#[macro_export]
macro_rules! cow {
    ($e:expr) => {
        std::borrow::Cow::Owned($e)
    };
}

/// A simple macro to make a new [`Cell`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use std::cell::Cell;
/// use sugars::cell;
/// # fn main() {
/// assert_eq!(Cell::new(10), cell!(10));
/// # }
/// ```
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
#[macro_export]
macro_rules! cell {
    ($e:expr) => {
        std::cell::Cell::new($e)
    };
    ($e:expr,) => {
        $crate::cell!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::cell!($e)),+,)
    };
}

/// A simple macro to make a new [`RefCell`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use std::cell::RefCell;
/// use sugars::refcell;
/// # fn main() {
/// assert_eq!(RefCell::new(10), refcell!(10));
/// # }
/// ```
///
/// [`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
#[macro_export]
macro_rules! refcell {
    ($e:expr) => {
        std::cell::RefCell::new($e)
    };
    ($e:expr,) => {
        $crate::refcell!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::refcell!($e)),+,)
    };
}

/// A simple macro to make a new [`Arc`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use std::sync::Arc;
/// use sugars::arc;
/// # fn main() {
/// assert_eq!(Arc::new(10), arc!(10));
/// # }
/// ```
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
#[macro_export]
macro_rules! arc {
    ($e:expr) => {
        std::sync::Arc::new($e)
    };
    ($e:expr,) => {
        $crate::arc!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::arc!($e)),+,)
    };
}

/// A simple macro to make a new [`Mutex`] value.
///
/// It is also able to create tuples if given more than one parameter.
///
/// # Example
/// ```
/// use std::sync::Mutex;
/// use sugars::mutex;
/// # fn main() {
/// let mutex = mutex!(String::new());
/// let mut locked = mutex.lock().unwrap();
/// (*locked).push_str("Hello World");
/// # }
/// ```
///
/// [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
#[macro_export]
macro_rules! mutex {
    ($e:expr) => {
        std::sync::Mutex::new($e)
    };
    ($e:expr,) => {
        $crate::mutex!($e)
    };
    ($($e:expr),+ $(,)?) => {
        ($($crate::mutex!($e)),+,)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn boxed() {
        assert_eq!(Box::new(10), boxed!(10));
        assert_eq!(Box::new(Some("String")), boxed!(Some("String")));
    }

    #[test]
    fn boxed_trailing_comma() {
        assert_eq!(Box::new(10), boxed!(10,));
        assert_eq!(Box::new(Some("String")), boxed!(Some("String"),));
    }

    #[test]
    fn boxed_tuples() {
        let expected1 = (Box::new(10), Box::new(11));
        let expected2 = (Box::new(Some("String")), Box::new(Some("other_str")));
        assert_eq!(expected1, boxed!(10, 11));
        assert_eq!(expected2, boxed!(Some("String"), Some("other_str")));
    }

    #[test]
    fn rc() {
        use std::rc::Rc;
        assert_eq!(Rc::new(10), rc!(10));
        assert_eq!(Rc::new(Some("String")), rc!(Some("String")));
    }

    #[test]
    fn rc_trailing_comma() {
        use std::rc::Rc;
        assert_eq!(Rc::new(10), rc!(10,));
        assert_eq!(Rc::new(Some("String")), rc!(Some("String"),));
    }

    #[test]
    fn rc_tuples() {
        use std::rc::Rc;
        let expected1 = (Rc::new(10), Rc::new(11));
        let expected2 = (Rc::new(Some("String")), Rc::new(Some("other_str")));
        assert_eq!(expected1, rc!(10, 11));
        assert_eq!(expected2, rc!(Some("String"), Some("other_str")));
    }

    #[test]
    fn cow() {
        use std::borrow::Cow;

        #[derive(Clone, PartialEq, Debug)]
        struct Test(i32);

        let expected: Cow<str> = Cow::Owned(String::from("Hello Cow"));
        let test: Cow<str> = cow!(String::from("Hello Cow"));

        assert_eq!(expected, test);

        let expected: Cow<Test> = Cow::Owned(Test(10));
        let test = cow!(Test(10));
        assert_eq!(expected, test);
    }

    #[test]
    fn cell() {
        use std::cell::Cell;
        assert_eq!(Cell::new(10), cell!(10));
        assert_eq!(Cell::new(Some("String")), cell!(Some("String")));
    }

    #[test]
    fn cell_trailing_comma() {
        use std::cell::Cell;
        assert_eq!(Cell::new(10), cell!(10,));
        assert_eq!(Cell::new(Some("String")), cell!(Some("String"),));
    }

    #[test]
    fn cell_tuples() {
        use std::cell::Cell;
        let expected1 = (Cell::new(10), Cell::new(11));
        let expected2 = (Cell::new(Some("String")), Cell::new(Some("other_str")));
        assert_eq!(expected1, cell!(10, 11));
        assert_eq!(expected2, cell!(Some("String"), Some("other_str")));
    }

    #[test]
    fn refcell() {
        use std::cell::RefCell;
        assert_eq!(RefCell::new(10), refcell!(10));
        assert_eq!(RefCell::new(Some("String")), refcell!(Some("String")));
    }

    #[test]
    fn refcell_trailing_comma() {
        use std::cell::RefCell;
        assert_eq!(RefCell::new(10), refcell!(10,));
        assert_eq!(RefCell::new(Some("String")), refcell!(Some("String"),));
    }

    #[test]
    fn refcell_tuples() {
        use std::cell::RefCell;
        let expected1 = (RefCell::new(10), RefCell::new(11));
        let expected2 = (RefCell::new(Some("String")), RefCell::new(Some("other_str")));
        assert_eq!(expected1, refcell!(10, 11));
        assert_eq!(expected2, refcell!(Some("String"), Some("other_str")));
    }

    #[test]
    fn arc() {
        use std::sync::Arc;
        assert_eq!(Arc::new(10), arc!(10));
        assert_eq!(Arc::new(Some("String")), arc!(Some("String")));
    }

    #[test]
    fn arc_trailing_comma() {
        use std::sync::Arc;
        assert_eq!(Arc::new(10), arc!(10,));
        assert_eq!(Arc::new(Some("String")), arc!(Some("String"),));
    }

    #[test]
    fn arc_tuples() {
        use std::sync::Arc;
        let expected1 = (Arc::new(10), Arc::new(11));
        let expected2 = (Arc::new(Some("String")), Arc::new(Some("other_str")));
        assert_eq!(expected1, arc!(10, 11));
        assert_eq!(expected2, arc!(Some("String"), Some("other_str")));
    }

    #[test]
    fn mutex() -> Result<(), Box<dyn std::error::Error>> {
        use std::sync::Mutex;

        let mutex_expected = Mutex::new(Some("String"));
        let mutex_test = mutex!(Some("String"));

        let expected = mutex_expected.lock().unwrap();
        let test = mutex_test.lock().unwrap();
        assert_eq!(expected.is_some(), test.is_some());

        Ok(())
    }
}
