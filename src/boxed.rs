//! A module related to the Box smart pointer

/// A simple macro to make a new Box value.
///
/// # Example
/// ```
/// use sugars::boxed;
/// fn main() {
///     assert_eq!(Box::new(10), boxed!(10));
/// }
#[macro_export]
macro_rules! boxed {
    ($e:expr) => (
        Box::new($e)
    );
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn boxed() {
        assert_eq!(Box::new(10), boxed!(10));
        assert_eq!(Box::new(Some("String")), boxed!(Some("String")));
    }
}
