//! Module for macros behaving as comprehensions

/// Macro to `Vec` collection comprehensions
///
/// Supports 2 types of grammars: Haskell-like and Python-like
///
/// **Limitations:** Only 3 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::cvec;
///
/// # fn main() {
/// // Haskell-like
/// let x = cvec![x; x <- 1..10];
/// let y = cvec![x; x <- 1..10, if x%2 == 0];
///
/// // Python-like
/// let w = cvec![x; for x in 1..10];
/// let z = cvec![x; for x in 1..10, if x%2 == 0];
/// # }
/// ```
#[macro_export]
macro_rules! cvec {
    ($e:expr; $i:ident <- $iter:expr) => {{
        $iter.map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; for $i:ident in $iter:expr) => {{
        $iter.map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; for $i:ident in $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e).collect::<Vec<_>>()
    }};

    // 2 nested (Iterative since my test show up that in this case iterative had best performance
    // and less headache about iteration and lifetimes)
    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr) => {{
        let mut v = Vec::new();
        for $i2 in $iter2 {
            for $i1 in $iter1 {
                v.push($e);
            }
        }
        v
    }};

    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i2 in $iter2 {
            for $i1 in $iter1 {
                if $cond {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; for $i1:ident in $iter1:expr, for $i2:ident in $iter2:expr) => {{
        let mut v = Vec::new();
        for $i2 in $iter2 {
            for $i1 in $iter1 {
                v.push($e);
            }
        }
        v
    }};

    ($e:expr; for $i1:ident in $iter1:expr, for $i2:ident in $iter2:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i2 in $iter2 {
            for $i1 in $iter1 {
                if $cond {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr, $i3:ident <- $iter3:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                for $i3 in $iter3 {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr, $i3:ident <- $iter3:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                for $i3 in $iter3 {
                    if $cond {
                        v.push($e);
                    }
                }
            }
        }
        v
    }};

    ($e:expr; for $i1:ident in $iter1:expr, for $i2:ident in $iter2:expr, for $i3:ident in $iter3:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                for $i3 in $iter3 {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; for $i1:ident in $iter1:expr, for $i2:ident in $iter2:expr, for $i3:ident in $iter3:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                for $i3 in $iter3 {
                    if $cond {
                        v.push($e);
                    }
                }
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
/// ```rust
/// use sugars::cmap;
///
/// # fn main() {
/// let a = 10;
/// // Haskell-like
/// let x = cmap!{x => x+a; x <- 1..10};
/// let y = cmap!{x => x+a; x <- 1..10, if x%2 == 0};
///
/// // Python-like
/// let w = cmap!{x => x+a; for x in 1..10};
/// let z = cmap!{x => x+a; for x in 1..10, if x%2 == 0};
/// # }
/// ```
#[macro_export]
macro_rules! cmap {
    ($key:expr => $value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::HashMap;
        $iter.map(|$i| ($key, $value)).collect::<HashMap<_, _>>()
    }};

    ($key:expr => $value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::HashMap;
        $iter
            .filter(|$i| $cond)
            .map(|$i| ($key, $value))
            .collect::<HashMap<_, _>>()
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::HashMap;
        $iter.map(|$p| ($key, $value)).collect::<HashMap<_, _>>()
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashMap;
        $iter
            .filter(|$p| $cond)
            .map(|$p| ($key, $value))
            .collect::<HashMap<_, _>>()
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
/// ```rust
/// use sugars::cset;
///
/// # fn main() {
/// // Haskell-like
/// let x = cset!{x; x <- 1..10};
/// let y = cset!{x; x <- 1..10, if x%2 == 0};
///
/// // Python-like
/// let w = cset!{x; for x in 1..10};
/// let z = cset!{x; for x in 1..10, if x%2 == 0};
/// # }
/// ```
#[macro_export]
macro_rules! cset {
    ($value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::HashSet;
        $iter.map(|$i| $value).collect::<HashSet<_>>()
    }};

    ($value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::HashSet;
        $iter
            .filter(|$i| $cond)
            .map(|$i| $value)
            .collect::<HashSet<_>>()
    }};

    ($value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::HashSet;
        $iter.map(|$p| $value).collect::<HashSet<_>>()
    }};

    ($value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashSet;
        $iter
            .filter(|$p| $cond)
            .map(|$p| $value)
            .collect::<HashSet<_>>()
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn cvec_haskell_basic_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; x <- 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_basic_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; x <- 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_2_nested_no_conditional() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; x <- y, y <- nested];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; x <- y, y <- nested, if x % 2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_3_nested_no_conditional() {
        let expected = vec![
            (1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (1, 2, 2),
            (1, 2, 3), (1, 2, 4), (1, 3, 3), (1, 3, 4), (1, 4, 4),
            (2, 2, 2), (2, 2, 3), (2, 2, 4), (2, 3, 3), (2, 3, 4),
            (2, 4, 4), (3, 3, 3), (3, 3, 4), (3, 4, 4), (4, 4, 4),
        ];
        let n: i32 = 4;
        let test = cvec![(x,y, z); x <- 1..=n, y <- x..=n, z <- y..=n];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_3_nested_with_conditional() {
        let expected = vec![(3, 4, 5), (6, 8, 10)];
        let n: i32 = 10;
        let test = cvec![(x, y, z); x <- 1..=n, y <- x..=n, z <- y..=n, if x.pow(2) + y.pow(2) == z.pow(2)];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_basic_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; for x in 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_basic_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; for x in 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_2_nested_no_conditional() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; for x in y, for y in nested];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; for x in y, for y in nested, if x % 2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_3_nested_no_conditional() {
        let expected = vec![
            (1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (1, 2, 2),
            (1, 2, 3), (1, 2, 4), (1, 3, 3), (1, 3, 4), (1, 4, 4),
            (2, 2, 2), (2, 2, 3), (2, 2, 4), (2, 3, 3), (2, 3, 4),
            (2, 4, 4), (3, 3, 3), (3, 3, 4), (3, 4, 4), (4, 4, 4),
        ];
        let n: i32 = 4;
        let test = cvec![(x, y, z); for x in 1..=n, for y in x..=n, for z in y..=n];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_3_nested_with_conditional() {
        let expected = vec![(3, 4, 5), (6, 8, 10)];
        let n: i32 = 10;
        let test = cvec![(x,y, z); for x in 1..=n, for y in x..=n, for z in y..=n, if x.pow(2) + y.pow(2) == z.pow(2)];

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
