//! Module for macros behaving as comprehensions

/// Generic lazy iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust,ignore
/// # use std::collections::*;
/// use sugars::c;
/// # fn main() {
/// let w: Vec<_> = c![x; x in 1..10].collect();
/// let z: HashSet<_> = c!{x; x in 1..10, if x%2 == 0}.collect();
/// # }
/// ```
#[macro_export]
macro_rules! c {
    ($e:expr; $i:pat in $iter:expr) => { $iter.map(|$i| $e) };

    ($e:expr; $i:pat in $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e)
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr) => {{
        $iter1.flat_map(|$i1| {
            $iter2.map(move |$i2| $e)
        })
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, if $cond:expr) => {{
        $iter1.flat_map(|$i1| {
            $iter2.filter_map(move |$i2| {
                if $cond {
                    Some($e)
                } else {
                    None
                }
            })
        })
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr) => {{
        $iter1.flat_map(|$i1| {
            $iter2.flat_map(move |$i2| {
                $iter3.map(move |$i3| $e)
            })
        })
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr, if $cond:expr) => {{
        $iter1.flat_map(|$i1| {
            $iter2.flat_map(move |$i2| {
                $iter3.filter_map(move |$i3| {
                    if $cond {
                        Some($e)
                    } else {
                        None
                    }
                })
            })
        })
    }};
}

/// Macro to [`Vec`] collection comprehensions
///
/// Supports 3 types of grammars: Haskell-like, Python-like and Just-`in`
///
/// ## Limitations
///  * Only 3 nested comprehensions
///  * Haskell-like cannot accept patterns
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
///
/// // Just-in or Implicit-For
/// let w = cvec![x; x in 1..10];
/// let z = cvec![x; x in 1..10, if x%2 == 0];
/// # }
/// ```
#[macro_export]
macro_rules! cvec {
    // Haskell like
    ($e:expr; $i:ident <- $iter:expr) => {{
        $iter.map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e).collect::<Vec<_>>()
    }};

    // 2 nested (Iterative since my test show up that in this case iterative had best performance
    // and less headache about iteration and lifetimes)
    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                v.push($e);
            }
        }
        v
    }};

    ($e:expr; $i1:ident <- $iter1:expr, $i2:ident <- $iter2:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
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

    // `for .. in` of Python-like
    ($e:expr; for $i:pat in $iter:expr) => {{
        $iter.map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; for $i:pat in $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; for $i1:pat in $iter1:expr, for $i2:pat in $iter2:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                v.push($e);
            }
        }
        v
    }};

    ($e:expr; for $i1:pat in $iter1:expr, for $i2:pat in $iter2:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                if $cond {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; for $i1:pat in $iter1:expr, for $i2:pat in $iter2:expr, for $i3:pat in $iter3:expr) => {{
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

    ($e:expr; for $i1:pat in $iter1:expr, for $i2:pat in $iter2:expr, for $i3:pat in $iter3:expr, if $cond:expr) => {{
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

    // Just `in`
    ($e:expr; $i:pat in $iter:expr) => {{
        $iter.map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; $i:pat in $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e).collect::<Vec<_>>()
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                v.push($e);
            }
        }
        v
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, if $cond:expr) => {{
        let mut v = Vec::new();
        for $i1 in $iter1 {
            for $i2 in $iter2 {
                if $cond {
                    v.push($e);
                }
            }
        }
        v
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr) => {{
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

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr, if $cond:expr) => {{
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

/// Macro to [`HashMap`] collection comprehensions
///
/// Supports 3 types of grammars: Haskell-like, Python-like, Just-`in`
///
/// ## Limitations
///  * Only 1 nested comprehensions
///  * Haskell-like cannot accept patterns
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
///
/// // Just-in or Implicit-For
/// let w = cmap!{x => x+a; x in 1..10};
/// let z = cmap!{x => x+a; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
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

    ($key:expr => $value:expr; $p:pat in $iter:expr) => {{
        use std::collections::HashMap;
        $iter.map(|$p| ($key, $value)).collect::<HashMap<_, _>>()
    }};

    ($key:expr => $value:expr; $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashMap;
        $iter
            .filter(|$p| $cond)
            .map(|$p| ($key, $value))
            .collect::<HashMap<_, _>>()
    }};

}

/// Macro to [`HashSet`] collection comprehensions
///
/// Supports 3 types of grammars: Haskell-like, Python-like and Just-`in`
///
/// ## Limitations
///  * Only 1 nested comprehensions
///  * Haskell-like cannot accept patterns
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

/// // Just-in or Implicit-for
/// let w = cset!{x; x in 1..10};
/// let z = cset!{x; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
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

    ($value:expr; $p:pat in $iter:expr) => {{
        use std::collections::HashSet;
        $iter.map(|$p| $value).collect::<HashSet<_>>()
    }};

    ($value:expr; $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::HashSet;
        $iter
            .filter(|$p| $cond)
            .map(|$p| $value)
            .collect::<HashSet<_>>()
    }};
}

/// Macro to [`BTreeMap`] collection comprehensions
///
/// Supports 3 types of grammars: Haskell-like, Python-like, Just-`in`
///
/// ## Limitations
///  * Only 1 nested comprehensions
///  * Haskell-like cannot accept patterns
///
/// # Examples:
/// ```rust
/// use sugars::cbtmap;
///
/// # fn main() {
/// let a = 10;
/// // Haskell-like
/// let x = cbtmap!{x => x+a; x <- 1..10};
/// let y = cbtmap!{x => x+a; x <- 1..10, if x%2 == 0};
///
/// // Python-like
/// let w = cbtmap!{x => x+a; for x in 1..10};
/// let z = cbtmap!{x => x+a; for x in 1..10, if x%2 == 0};
///
/// // Just-in
/// let w = cbtmap!{x => x+a; x in 1..10};
/// let z = cbtmap!{x => x+a; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BtreeMap.html
#[macro_export]
macro_rules! cbtmap {
    ($key:expr => $value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::BTreeMap;
        $iter.map(|$i| ($key, $value)).collect::<BTreeMap<_, _>>()
    }};

    ($key:expr => $value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeMap;
        $iter
            .filter(|$i| $cond)
            .map(|$i| ($key, $value))
            .collect::<BTreeMap<_, _>>()
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::BTreeMap;
        $iter.map(|$p| ($key, $value)).collect::<BTreeMap<_, _>>()
    }};

    ($key:expr => $value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeMap;
        $iter
            .filter(|$p| $cond)
            .map(|$p| ($key, $value))
            .collect::<BTreeMap<_, _>>()
    }};

    ($key:expr => $value:expr; $p:pat in $iter:expr) => {{
        use std::collections::BTreeMap;
        $iter.map(|$p| ($key, $value)).collect::<BTreeMap<_, _>>()
    }};

    ($key:expr => $value:expr; $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeMap;
        $iter
            .filter(|$p| $cond)
            .map(|$p| ($key, $value))
            .collect::<BTreeMap<_, _>>()
    }};
}

/// Macro to [`BTreeSet`] collection comprehensions
///
/// Supports 3 types of grammars: Haskell-like, Python-like or Just-`in`
///
/// ## Limitations
///  * Only 1 nested comprehensions
///  * Haskell-like cannot accept patterns
///
/// # Examples:
/// ```rust
/// use sugars::cbtset;
///
/// # fn main() {
/// // Haskell-like
/// let x = cbtset!{x; x <- 1..10};
/// let y = cbtset!{x; x <- 1..10, if x%2 == 0};
///
/// // Python-like
/// let w = cbtset!{x; for x in 1..10};
/// let z = cbtset!{x; for x in 1..10, if x%2 == 0};
///
/// // Just-in
/// let w = cbtset!{x; x in 1..10};
/// let z = cbtset!{x; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BtreeSet.html
#[macro_export]
macro_rules! cbtset {
    ($value:expr; $i:ident <- $iter:expr) => {{
        use std::collections::BTreeSet;
        $iter.map(|$i| $value).collect::<BTreeSet<_>>()
    }};

    ($value:expr; $i:ident <- $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeSet;
        $iter
            .filter(|$i| $cond)
            .map(|$i| $value)
            .collect::<BTreeSet<_>>()
    }};

    ($value:expr; for $p:pat in $iter:expr) => {{
        use std::collections::BTreeSet;
        $iter.map(|$p| $value).collect::<BTreeSet<_>>()
    }};

    ($value:expr; for $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeSet;
        $iter
            .filter(|$p| $cond)
            .map(|$p| $value)
            .collect::<BTreeSet<_>>()
    }};

    ($value:expr; $p:pat in $iter:expr) => {{
        use std::collections::BTreeSet;
        $iter.map(|$p| $value).collect::<BTreeSet<_>>()
    }};

    ($value:expr; $p:pat in $iter:expr, if $cond:expr) => {{
        use std::collections::BTreeSet;
        $iter
            .filter(|$p| $cond)
            .map(|$p| $value)
            .collect::<BTreeSet<_>>()
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

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
        let test = cvec![x; y <- nested, x <- y];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_haskell_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; y <- nested, x <- y, if x % 2 == 0];

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
        let test = cvec![(x, y, z); x <- 1..=n, y <- x..=n, z <- y..=n];

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
        let test = cvec![x; for y in nested, for x in y];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_python_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; for y in nested, for x in y, if x % 2 == 0];

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
    fn cvec_just_in_basic_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; x in 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_basic_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; x in 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_2_nested_no_conditional() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; y in nested, x in y];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; y in nested, x in y, if x % 2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_3_nested_no_conditional() {
        let expected = vec![
            (1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (1, 2, 2),
            (1, 2, 3), (1, 2, 4), (1, 3, 3), (1, 3, 4), (1, 4, 4),
            (2, 2, 2), (2, 2, 3), (2, 2, 4), (2, 3, 3), (2, 3, 4),
            (2, 4, 4), (3, 3, 3), (3, 3, 4), (3, 4, 4), (4, 4, 4),
        ];
        let n: i32 = 4;
        let test = cvec![(x, y, z); x in 1..=n, y in x..=n, z in y..=n];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_3_nested_with_conditional() {
        let expected = vec![(3, 4, 5), (6, 8, 10)];
        let n: i32 = 10;
        let test = cvec![(x,y, z); x in 1..=n, y in x..=n, z in y..=n, if x.pow(2) + y.pow(2) == z.pow(2)];

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_haskell_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap!{x => x+a; x <- 1..10};

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
        let test = cmap!{x => x+a; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_python_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap!{x => x+a; for x in 1..10};

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
        let test = cmap!{x => x+a; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_just_in_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap!{x => x+a; x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_just_in_with_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cmap!{x => x+a; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_haskell_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset!{x; x <- 1..10};

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
        let test = cset!{x; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_python_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset!{x; for x in 1..10};

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
        let test = cset!{x; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_just_in_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset!{x; x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_just_in_with_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cset!{x; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_haskell_no_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cbtmap!{x => x+a; x <- 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_haskell_with_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cbtmap!{x => x+a; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_python_no_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cbtmap!{x => x+a; for x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_python_with_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cbtmap!{x => x+a; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_just_in_no_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cbtmap!{x => x+a; x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_just_in_with_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i, i + a);
            }
        }
        let test = cbtmap!{x => x+a; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_haskell_no_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cbtset!{x; x <- 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_haskell_with_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cbtset!{x; x <- 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_python_no_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cbtset!{x; for x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_python_with_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cbtset!{x; for x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_just_in_no_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cbtset!{x; x in 1..10};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_just_in_with_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            if i % 2 == 0 {
                expected.insert(i);
            }
        }
        let test = cbtset!{x; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }
}
