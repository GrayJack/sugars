/// Macro that return the hash of what is passed and also can receive
/// a hasher to use that intead of default `HashMap` Hasher.
///
/// # Example
/// ```
/// # use std::collections::hash_map::DefaultHasher;
/// # use sugars::hash;
/// # fn main() {
/// // No Hasher
/// let hash = hash!("a");
/// assert_eq!(8_186_225_505_942_432_243, hash);
///
/// // With Hasher
/// let hash = hash!("b", DefaultHasher::new());
/// assert_eq!(16_993_177_596_579_750_922, hash);
/// # }
/// ```
#[macro_export]
macro_rules! hash {
    ($e:expr) => ({
        use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
        let mut hasher = DefaultHasher::new();
        $e.hash(&mut hasher);
        hasher.finish()
    });

    ($e:expr, $hasher:expr) => ({
        use std::{hash::{Hash, Hasher}};
        let mut hasher = $hasher;
        $e.hash(&mut hasher);
        hasher.finish()
    })
}


#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn hash_default() {
        let a = "b";
        let expected = 16_993_177_596_579_750_922;
        let test = hash!(a);

        assert_eq!(expected, test);
    }

    #[test]
    fn hash_with_hasher() {
        let a = "b";
        let expected = 16_993_177_596_579_750_922;
        let test = hash!(a, DefaultHasher::new());

        assert_eq!(expected, test);
    }
}
