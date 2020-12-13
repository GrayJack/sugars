//! Module for macros behaving as comprehensions

/// Generic lazy iterator comprehensions.
///
/// Nice to have when what you collecting are not in `std::collections` or not in the `std` at all.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust
/// # use std::collections::*;
/// use sugars::c;
/// let w: Vec<_> = c![x; x in 1..10].collect();
/// let z: HashSet<_> = c!{x; x in 1..10, if x%2 == 0}.collect();
/// ```
#[macro_export]
macro_rules! c {
    ($e:expr; $i:pat in $iter:expr) => {
        $iter.map(|$i| $e)
    };

    ($e:expr; $i:pat in $iter:expr, if $cond:expr) => {{
        $iter.filter(|$i| $cond).map(|$i| $e)
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr) => {{
        $iter1.flat_map(|$i1| $iter2.map(move |$i2| $e))
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, if $cond:expr) => {{
        $iter1.flat_map(|$i1| $iter2.filter_map(move |$i2| if $cond { Some($e) } else { None }))
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr) => {{
        $iter1.flat_map(|$i1| $iter2.flat_map(move |$i2| $iter3.map(move |$i3| $e)))
    }};

    ($e:expr; $i1:pat in $iter1:expr, $i2:pat in $iter2:expr, $i3:pat in $iter3:expr, if $cond:expr) => {{
        $iter1.flat_map(|$i1| {
            $iter2.flat_map(move |$i2| {
                $iter3.filter_map(move |$i3| if $cond { Some($e) } else { None })
            })
        })
    }};
}

/// Build [`Vec`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::cvec;
///
/// # fn main() {
/// let w = cvec![x; x in 1..10];
/// let z = cvec![x; x in 1..10, if x%2 == 0];
/// # }
/// ```
#[macro_export]
macro_rules! cvec {
    ($($tokens: tt)+) => {
        $crate::c![$($tokens)+].collect::<::std::vec::Vec<_>>()
    };
}

/// Build [`VecDeque`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::cdeque;
///
/// # fn main() {
/// let w = cdeque![x; x in 1..10];
/// let z = cdeque![x; x in 1..10, if x%2 == 0];
/// # }
/// ```
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
#[macro_export]
macro_rules! cdeque {
    ($($tokens: tt)+) => {{
        use std::collections::VecDeque;
        $crate::c![$($tokens)+].collect::<::std::collections::VecDeque<_>>()
    }};
}

/// Build [`LinkedList`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::clkl;
///
/// # fn main() {
/// let w = clkl![x; x in 1..10];
/// let z = clkl![x; x in 1..10, if x%2 == 0];
/// # }
/// ```
///
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
#[macro_export]
macro_rules! clkl {
    ($($tokens: tt)+) => {{
        use std::collections::LinkedList;
        $crate::c![$($tokens)+].collect::<::std::collections::LinkedList<_>>()
    }};
}

/// Build [`BinaryHeap`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```
/// use sugars::cbheap;
///
/// # fn main() {
/// let w = cbheap![x; x in 1..10];
/// let z = cbheap![x; x in 1..10, if x%2 == 0];
/// # }
/// ```
///
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
#[macro_export]
macro_rules! cbheap {
    ($($tokens: tt)+) => {{
        use std::collections::BinaryHeap;
        $crate::c![$($tokens)+].collect::<::std::collections::BinaryHeap<_>>()
    }};
}

/// Build [`HashMap`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust
/// use sugars::cmap;
///
/// # fn main() {
/// let a = 10;
/// let w = cmap!{x => x+a; x in 1..10};
/// let z = cmap!{x => x+a; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
#[macro_export]
macro_rules! cmap {
    ($key:expr => $value:expr; $($tokens: tt)+) => {{
        use std::collections::HashMap;
        $crate::c![ ($key, $value); $($tokens)+ ].collect::<::std::collections::HashMap<_, _>>()
    }};
}

/// Build [`HashSet`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust
/// use sugars::cset;
///
/// # fn main() {
/// let w = cset!{x; x in 1..10};
/// let z = cset!{x; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
#[macro_export]
macro_rules! cset {
    ($($tokens: tt)+) => {{
        use std::collections::HashSet;
        $crate::c![$($tokens)+].collect::<::std::collections::HashSet<_>>()
    }};
}

/// Build [`BTreeMap`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust
/// use sugars::cbtmap;
///
/// # fn main() {
/// let a = 10;
/// let w = cbtmap!{x => x+a; x in 1..10};
/// let z = cbtmap!{x => x+a; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BtreeMap.html
#[macro_export]
macro_rules! cbtmap {
    ($key:expr => $value:expr; $($tokens: tt)+) => {{
        $crate::c![ ($key, $value); $($tokens)+ ].collect::<::std::collections::BTreeMap<_, _>>()
    }};
}

/// Build [`BTreeSet`] from collection iterator comprehensions.
///
/// ## Limitations
///  * Only 3 nested comprehensions
///
/// # Examples:
/// ```rust
/// use sugars::cbtset;
///
/// # fn main() {
/// let w = cbtset!{x; x in 1..10};
/// let z = cbtset!{x; x in 1..10, if x%2 == 0};
/// # }
/// ```
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BtreeSet.html
#[macro_export]
macro_rules! cbtset {
    ($($tokens: tt)+) => {{
        $crate::c![$($tokens)+].collect::<::std::collections::BTreeSet<_>>()
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    #[test]
    fn c_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test: Vec<_> = c![x*2; x in 1..5].collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn c_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test: Vec<_> = c![x; x in 0..10, if x%2 == 0].collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn c_2_nested_no_conditional() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test: Vec<_> = c![x; y in nested.into_iter(), x in y.into_iter()].collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn c_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test: Vec<_> =
            c![x; y in nested.into_iter(), x in y.into_iter(), if x % 2 == 0].collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn c_3_nested_no_conditional() {
        let expected = vec![
            (1, 1, 1),
            (1, 1, 2),
            (1, 1, 3),
            (1, 1, 4),
            (1, 2, 2),
            (1, 2, 3),
            (1, 2, 4),
            (1, 3, 3),
            (1, 3, 4),
            (1, 4, 4),
            (2, 2, 2),
            (2, 2, 3),
            (2, 2, 4),
            (2, 3, 3),
            (2, 3, 4),
            (2, 4, 4),
            (3, 3, 3),
            (3, 3, 4),
            (3, 4, 4),
            (4, 4, 4),
        ];
        let n: i32 = 4;
        let test: Vec<_> = c![(x, y, z); x in 1..=n, y in x..=n, z in y..=n].collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn c_3_nested_with_conditional() {
        let expected = vec![(3, 4, 5), (6, 8, 10)];
        let n: i32 = 10;
        let test: Vec<_> =
            c![(x,y, z); x in 1..=n, y in x..=n, z in y..=n, if x.pow(2) + y.pow(2) == z.pow(2)]
                .collect();

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_basic_no_conditional() {
        let expected = vec![2, 4, 6, 8];
        let test = cvec![x*2; x in 1..5];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_basic_with_conditional() {
        let expected = vec![0, 2, 4, 6, 8];
        let test = cvec![x; x in 0..10, if x%2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_2_nested_no_conditional() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; y in nested.into_iter(), x in y.into_iter()];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_2_nested_with_conditional() {
        let expected = vec![2, 4, 6, 8];
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let test = cvec![x; y in nested.into_iter(), x in y.into_iter(), if x % 2 == 0];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_3_nested_no_conditional() {
        let expected = vec![
            (1, 1, 1),
            (1, 1, 2),
            (1, 1, 3),
            (1, 1, 4),
            (1, 2, 2),
            (1, 2, 3),
            (1, 2, 4),
            (1, 3, 3),
            (1, 3, 4),
            (1, 4, 4),
            (2, 2, 2),
            (2, 2, 3),
            (2, 2, 4),
            (2, 3, 3),
            (2, 3, 4),
            (2, 4, 4),
            (3, 3, 3),
            (3, 3, 4),
            (3, 4, 4),
            (4, 4, 4),
        ];
        let n: i32 = 4;
        let test = cvec![(x, y, z); x in 1..=n, y in x..=n, z in y..=n];

        assert_eq!(expected, test);
    }

    #[test]
    fn cvec_just_in_3_nested_with_conditional() {
        let expected = vec![(3, 4, 5), (6, 8, 10)];
        let n: i32 = 10;
        let test =
            cvec![(x,y, z); x in 1..=n, y in x..=n, z in y..=n, if x.pow(2) + y.pow(2) == z.pow(2)];

        assert_eq!(expected, test);
    }

    #[test]
    fn cmap_just_in_no_conditional() {
        let a = 10;
        let mut expected = HashMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cmap! {x => x+a; x in 1..10};

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
        let test = cmap! {x => x+a; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cset_just_in_no_conditional() {
        let mut expected = HashSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cset! {x; x in 1..10};

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
        let test = cset! {x; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtmap_just_in_no_conditional() {
        let a = 10;
        let mut expected = BTreeMap::new();
        for i in 1..10 {
            expected.insert(i, i + a);
        }
        let test = cbtmap! {x => x+a; x in 1..10};

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
        let test = cbtmap! {x => x+a; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }

    #[test]
    fn cbtset_just_in_no_conditional() {
        let mut expected = BTreeSet::new();
        for i in 1..10 {
            expected.insert(i);
        }
        let test = cbtset! {x; x in 1..10};

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
        let test = cbtset! {x; x in 1..10, if x%2==0};

        assert_eq!(expected, test);
    }
}
