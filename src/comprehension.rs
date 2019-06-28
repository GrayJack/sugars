//! Module for macros behaving as comprehensions

/// Macro to `Vec` collection comprehensions
///
/// Supports 2 types of grammars: Haskell-like and Python-like
///
/// **Limitations:** Only 2 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::cvec;
/// fn main() {
///     // Haskell-like
///     let x = cvec![x; x <- 1..10];
///     let y = cvec![x; x <- 1..10, if x%2 == 0];
///
///     // Python-like
///     let w = cvec![x; for x in 1..10];
///     let z = cvec![x; for x in 1..10, if x%2 == 0];
/// }
/// ```
#[macro_export]
macro_rules! cvec {
    ($e:expr; $i:ident <- $iter:expr) => {{
        let mut v = Vec::new();
        for $i in $iter {
            v.push($e.clone());
        }
        v
    }};

    ($e:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i in $iter {
            if $cond {
                v.push($e.clone());
            }
        }
        v
    }};

    ($e:expr; for $i:ident in $iter:expr) => {{
        let mut v = Vec::new();
        for $i in $iter {
            v.push($e.clone());
        }
        v
    }};

    ($e:expr; for $i:ident in $iter:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i in $iter {
            if $cond {
                v.push($e.clone());
            }
        }
        v
    }};
}

/// Macro to `HashMap` collection comprehensions
///
/// Supports 2 types of grammars: Haskell-like and Python-like
///
/// **Limitations:** Only 1 nested comprehensions and
/// pattern support only works with python syntax
///
/// # Examples:
/// ```
/// use sugars::cmap;
/// fn main() {
///     let a = 10;
///     // Haskell-like
///     let x = cmap!{x => x+a; x <- 1..10};
///     let y = cmap!{x => x+a; x <- 1..10, if x%2 == 0};
///
///     // Python-like
///     let w = cmap!{x => x+a; for x in 1..10};
///     let z = cmap!{x => x+a; for x in 1..10, if x%2 == 0};
/// }
/// ```
#[macro_export]
macro_rules! cmap {
    ($key:expr => $value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for $i in $iter {
            map.insert($key, $value);
        }
        map
    }};

    ($key:expr => $value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for $i in $iter {
            if $cond {
                map.insert($key, $value);
            }
        }
        map
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for $p in $iter {
            map.insert($key, $value);
        }
        map
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for $p in $iter {
            if $cond {
                map.insert($key, $value);
            }
        }
        map
    }};
}

/// Macro to `HashSet` collection comprehensions
///
/// Supports 2 types of grammars: Haskell-like and Python-like
///
/// **Limitations:** Only 1 nested comprehensions and
/// pattern support only works with python syntax
///
/// # Examples:
/// ```
/// use sugars::cset;
/// fn main() {
///     // Haskell-like
///     let x = cset!{x; x <- 1..10};
///     let y = cset!{x; x <- 1..10, if x%2 == 0};
///
///     // Python-like
///     let w = cset!{x; for x in 1..10};
///     let z = cset!{x; for x in 1..10, if x%2 == 0};
/// }
/// ```
#[macro_export]
macro_rules! cset {
    ($value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for $i in $iter {
            set.insert($value);
        }
        set
    }};

    ($value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for $i in $iter {
            if $cond {
                set.insert($value);
            }
        }
        set
    }};

    ($value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for $p in $iter {
            set.insert($value);
        }
        set
    }};

    ($value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for $p in $iter {
            if $cond {
                set.insert($value);
            }
        }
        set
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn cvec_haskell_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; x <- 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; x <- 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; for x in 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; for x in 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_haskell_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap! {x => x+a; x <- 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_haskell_with_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cmap! {x => x+a; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_python_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap! {x => x+a; for x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_python_with_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cmap! {x => x+a; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_haskell_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset! {x; x <- 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_haskell_with_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cset! {x; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_python_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset! {x; for x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_python_with_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cset! {x; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }
}
