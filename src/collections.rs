//! Module for collections literal macros

/// Create a `HashMap` from a list of key-value pairs
///
/// # Example
///
/// ```rust
/// use sugars::hmap;
///
/// # fn main() {
/// let map = hmap!{
///     "a" => 1,
///     "b" => 2,
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
#[macro_export]
macro_rules! hmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => ({
            let cap = hmap!(@count $($key),*);
            let mut map = ::std::collections::HashMap::with_capacity(cap);
            $(
                let _ = map.insert($key, $value);
            )*
            map
    });
}

/// Create a `HashSet` from a list of elements.
///
/// # Example
///
/// ```rust
/// use sugars::hset;
///
/// # fn main() {
/// let set = hset!{"a", "b"};
///
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! hset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hset!(@single $rest)),*]));

    ($($key:expr,)+) => { hset!($($key),+) };
    ($($key:expr),*) => ({
        let cap = hset!(@count $($key),*);
        let mut set = ::std::collections::HashSet::with_capacity(cap);
        $(
            let _ = set.insert($key);
        )*
        set
    });
}

/// Create a `BTreeMap` from a list of key-value pairs
///
/// # Example
///
/// ```rust
/// use sugars::btmap;
///
/// # fn main() {
/// let map = btmap!{
///     "a" => 1,
///     "b" => 2,
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
#[macro_export]
macro_rules! btmap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btmap!($($key => $value),+));

    ( $($key:expr => $value:expr),* ) => ({
        let mut map = std::collections::BTreeMap::new();
        $(
            let _ = map.insert($key, $value);
        )*
        map
    });
}

/// Create a `BTreeSet` from a list of elements.
///
/// # Example
///
/// ```rust
/// use sugars::btset;
///
/// # fn main() {
/// let set = btset!{"a", "b"};
///
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! btset {
    ($($key:expr,)+) => (btset!($($key),+));

    ( $($key:expr),* ) => ({
        let mut set = std::collections::BTreeSet::new();
        $(
            set.insert($key);
        )*
        set
    });
}

/// Create a `LinkedList` from a list of elements. It pushes the element to the back of the list.
///
/// # Example
///
/// ```rust
/// use sugars::lkl;
///
/// # fn main() {
/// let lkl = lkl!["a", "b"];
///
/// assert!(lkl.contains(&"a"));
/// assert!(lkl.contains(&"b"));
/// assert!(!lkl.contains(&"c"));
/// # }
/// ```
#[macro_export]
macro_rules! lkl {
    ($($key:expr,)+) => (lkl!($($key),+));

    ( $($key:expr),* ) => {{
        let mut lkl = std::collections::LinkedList::new();
        $(
            lkl.push_back($key);
        )*
        lkl
    }};
}

/// Create a `LinkedList` from a list of elements. It pushes the element to the start of the list.
///
/// # Example
///
/// ```rust
/// use sugars::lkl;
///
/// # fn main() {
/// let lkl = lkl!["a", "b"];
///
/// assert!(lkl.contains(&"a"));
/// assert!(lkl.contains(&"b"));
/// assert!(!lkl.contains(&"c"));
/// # }
/// ```
#[macro_export]
macro_rules! flkl {
    ($($key:expr,)+) => (flkl!($($key),+));

    ( $($key:expr),* ) => {{
        let mut lkl = std::collections::LinkedList::new();
        $(
            lkl.push_front($key);
        )*
        lkl
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn hmap() {
        let map = hmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map.get("c"), None);
    }

    #[test]
    fn hset() {
        let set = hset! {"a", "b"};
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(!set.contains("c"));
    }

    #[test]
    fn btmap() {
        let map = btmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map.get("c"), None);
    }

    #[test]
    fn btset() {
        let set = btset! {"a", "b"};
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(!set.contains("c"));
    }

    #[test]
    fn lkl() {
        use std::collections::LinkedList;
        let mut expected = LinkedList::new();
        expected.push_back("a");
        expected.push_back("b");
        let lkl = lkl!["a", "b"];

        assert!(lkl.contains(&"a"));
        assert!(lkl.contains(&"b"));
        assert!(!lkl.contains(&"c"));
        assert_eq!(expected, lkl);
    }

    #[test]
    fn flkl() {
        use std::collections::LinkedList;
        let mut expected = LinkedList::new();
        expected.push_front("a");
        expected.push_front("b");
        let lkl = flkl!["a", "b"];

        assert!(lkl.contains(&"a"));
        assert!(lkl.contains(&"b"));
        assert!(!lkl.contains(&"c"));
        assert_eq!(expected, lkl);
    }
}
