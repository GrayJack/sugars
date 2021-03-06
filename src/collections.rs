//! Module for collections literal macros.

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    (@subst $($x: tt)*) => (());
    ($($rest: expr),*) => (<[()]>::len(&[$($crate::count!(@subst $rest)),*]));
}

/// Create a [`HashMap`] from a list of key-value pairs.
///
/// # Example
///
/// ```rust
/// use sugars::hmap;
///
/// # fn main() {
/// let map = hmap! {
///     "a" => 1,
///     "b" => 2,
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
///
/// [`HashMap`]: std::collections::HashMap
#[macro_export]
macro_rules! hmap {
    () => { ::std::collections::HashMap::new() };

    ( $($key: expr => $value: expr),+ $(,)? ) => {{
            const CAP: usize = $crate::count!($($key),*);
            let mut map = ::std::collections::HashMap::with_capacity(CAP);
            $(
                let _ = map.insert($key, $value);
            )+
            map
    }};
}

/// Create a [`HashSet`] from a list of elements.
///
/// # Example
///
/// ```rust
/// use sugars::hset;
///
/// # fn main() {
/// let set = hset! {"a", "b"};
///
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
///
/// [`HashSet`]: std::collections::HashMap
#[macro_export]
macro_rules! hset {
    () => { ::std::collections::HashSet::new() };

    ($($elem: expr),+ $(,)?) => {{
        const CAP: usize = $crate::count!($($elem),*);
        let mut set = ::std::collections::HashSet::with_capacity(CAP);
        $(
            let _ = set.insert($elem);
        )+
        set
    }};
}

/// Create a [`BTreeMap`] from a list of key-value pairs.
///
/// # Example
///
/// ```rust
/// use sugars::btmap;
///
/// # fn main() {
/// let map = btmap! {
///     "a" => 1,
///     "b" => 2,
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
///
/// [`BTreeMap`]: std::collections::BTreeMap
#[macro_export]
macro_rules! btmap {
    () => { ::std::collections::BTreeMap::new() };

    ( $($key: expr => $value: expr),+ $(,)? ) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $(
            let _ = map.insert($key, $value);
        )+
        map
    }};
}

/// Create a [`BTreeSet`] from a list of elements.
///
/// # Example
///
/// ```rust
/// use sugars::btset;
///
/// # fn main() {
/// let set = btset! {"a", "b"};
///
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// let mut iter = set.iter();
/// assert_eq!(Some(&"a"), iter.next());
/// assert_eq!(Some(&"b"), iter.next());
/// assert_eq!(None, iter.next());
/// # }
/// ```
///
/// [`BTreeSet`]: std::collections::BTreeSet
#[macro_export]
macro_rules! btset {
    () => { ::std::collections::BTreeSet::new() };

    ( $($elem: expr),+ $(,)? ) => {{
        let mut set = ::std::collections::BTreeSet::new();
        $(
            set.insert($elem);
        )+
        set
    }};
}

/// Create a [`VecDeque`] from a list of elements.
///
/// # Examples
///
/// ```rust
/// use sugars::deque;
/// # use std::collections::VecDeque;
///
/// # fn main() {
/// let deque = deque![1, 2, 3, 4];
/// let deque2: VecDeque<_> = (1..=4).collect();
///
/// assert_eq!(deque, deque2);
/// # }
/// ```
///
/// [`VecDeque`]: std::collections::VecDeque
#[macro_export]
macro_rules! deque {
    () => { ::std::collections::VecDeque::new() };

    ($elem: expr; $n: expr) => {{
        let mut deque = ::std::collections::VecDeque::new();
        deque.resize_with($n, || $elem);
        deque
    }};

    ( $($elem: expr),+ $(,)? ) => {{
        const CAP: usize = $crate::count!($($elem),*);
        let mut deque = ::std::collections::VecDeque::with_capacity(CAP);
        $(
            deque.push_back($elem);
        )+
        deque
    }};
}

/// Create a [`LinkedList`] from a list of elements.
///
/// # Examples
///
/// ```rust
/// use sugars::lkl;
/// # fn main() {
/// let lkl = lkl!["a", "b"];
///
/// assert!(lkl.contains(&"a"));
/// assert!(lkl.contains(&"b"));
/// assert!(!lkl.contains(&"c"));
///
/// let mut iter = lkl.iter();
/// assert_eq!(Some(&"a"), iter.next());
/// assert_eq!(Some(&"b"), iter.next());
/// assert_eq!(None, iter.next());
/// # }
/// ```
///
/// If you want to build a [`LinkedList`] with repeated values:
/// ```rust
/// use sugars::lkl;
/// # fn main() {
/// let lkl = lkl!["10"; 3];
///
/// let mut iter = lkl.iter();
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(None, iter.next());
/// # }
/// ```
///
/// [`LinkedList`]: std::collections::LinkedList
#[macro_export]
macro_rules! lkl {
    () => { ::std::collections::LinkedList::new() };

    ($elem: expr; $n: expr) => {{
        let mut lkl = ::std::collections::LinkedList::new();
        (0..$n).for_each(|_| lkl.push_back($elem));
        lkl
    }};

    ( $($elem: expr),+ $(,)? ) => {{
        let mut lkl = ::std::collections::LinkedList::new();
        $(
            lkl.push_back($elem);
        )*
        lkl
    }};
}

/// Create a reversed [`LinkedList`] from a list of elements.
///
/// # Examples
///
/// ```rust
/// use sugars::rlkl;
/// # fn main() {
/// let lkl = rlkl!["a", "b"];
///
/// assert!(lkl.contains(&"a"));
/// assert!(lkl.contains(&"b"));
/// assert!(!lkl.contains(&"c"));
///
/// let mut iter = lkl.iter();
/// assert_eq!(Some(&"b"), iter.next());
/// assert_eq!(Some(&"a"), iter.next());
/// assert_eq!(None, iter.next());
/// # }
/// ```
///
/// If you want to build a [`LinkedList`] with repeated values:
/// ```rust
/// use sugars::rlkl;
/// # fn main() {
/// let lkl = rlkl!["10"; 3];
///
/// let mut iter = lkl.iter();
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(Some(&"10"), iter.next());
/// assert_eq!(None, iter.next());
/// # }
/// ```
///
/// [`LinkedList`]: std::collections::LinkedList
#[macro_export]
macro_rules! rlkl {
    () => { ::std::collections::LinkedList::new() };

    ($elem: expr; $n: expr) => {{
        let mut lkl = ::std::collections::LinkedList::new();
        (0..$n).for_each(|_| lkl.push_front($elem));
        lkl
    }};

    ( $($elem: expr),+ $(,)? ) => {{
        let mut lkl = ::std::collections::LinkedList::new();
        $(
            lkl.push_front($elem);
        )*
        lkl
    }};
}

/// Create a [`BinaryHeap`] from a list of elements.
///
/// # Examples
///
/// ```rust
/// use sugars::bheap;
/// # fn main() {
/// let mut heap = bheap![4, 1, 3, 2];
///
/// assert_eq!(Some(4), heap.pop());
/// assert_eq!(Some(3), heap.pop());
/// assert_eq!(Some(2), heap.pop());
/// assert_eq!(Some(1), heap.pop());
/// assert_eq!(None, heap.pop());
/// # }
/// ```
///
/// [`BinaryHeap`]: std::collections::BinaryHeap
#[macro_export]
macro_rules! bheap {
    () => { ::std::collections::BinaryHeap::new() };

    ( $($elem: expr),+ $(,)? ) => {{
        const CAP: usize = $crate::count!($($elem),*);
        let mut bheap = ::std::collections::BinaryHeap::with_capacity(CAP);
        $(
            bheap.push($elem);
        )+

        bheap
    }}
}

#[cfg(test)]
mod tests {
    use std::collections::*;

    #[test]
    fn hmap() {
        let map = hmap! {
            "a" => 1,
            "b" => 2,
        };
        let map2: HashMap<&str, i32, _> = hmap! {};
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map.get("c"), None);

        assert!(map2.is_empty());
    }

    #[test]
    fn hset() {
        let set = hset! {"a", "b"};
        let set2: HashSet<&str> = hset! {};
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(!set.contains("c"));

        assert!(set2.is_empty());
    }

    #[test]
    fn btmap() {
        let map = btmap! {
            "a" => 1,
            "b" => 2,
        };
        let map2: BTreeMap<&str, i32> = btmap! {};
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map.get("c"), None);

        assert!(map2.is_empty());
    }

    #[test]
    fn btset() {
        let set = btset! {"a", "b"};
        let set2: BTreeSet<&str> = btset! {};
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(!set.contains("c"));

        assert!(set2.is_empty());
    }

    #[test]
    fn deque() {
        let deque: VecDeque<i32> = deque![];
        assert!(deque.is_empty());

        let deque1 = deque![0; 7];
        let deque1_test: VecDeque<i32> = std::iter::repeat(0).take(7).collect();
        assert_eq!(deque1_test, deque1);

        let deque2 = deque![0, 1, 2, 3, 4, 5];
        let deque2_test: VecDeque<_> = (0..=5).collect();
        assert_eq!(deque2_test, deque2);
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

        const LKL2: LinkedList<&str> = lkl![];
        assert!(LKL2.is_empty());
    }

    #[test]
    fn lkl_repeat() {
        use std::collections::LinkedList;
        let expected = {
            let mut l = LinkedList::new();
            l.push_back("a");
            l.push_back("a");
            l
        };
        let test = lkl!["a"; 2];

        assert_eq!(expected, test);
    }

    #[test]
    fn rlkl() {
        use std::collections::LinkedList;
        let mut expected = LinkedList::new();
        expected.push_front("a");
        expected.push_front("b");
        let lkl = rlkl!["a", "b"];

        assert!(lkl.contains(&"a"));
        assert!(lkl.contains(&"b"));
        assert!(!lkl.contains(&"c"));
        assert_eq!(expected, lkl);

        const LKL2: LinkedList<&str> = rlkl![];
        assert!(LKL2.is_empty());
    }

    #[test]
    fn rlkl_repeat() {
        use std::collections::LinkedList;
        let expected = {
            let mut l = LinkedList::new();
            l.push_front("a");
            l.push_front("a");
            l
        };
        let test = rlkl!["a"; 2];

        assert_eq!(expected, test);
    }

    #[test]
    fn bheap() {
        let bheap1: BinaryHeap<()> = bheap![];
        assert!(bheap1.is_empty());

        let bheap2 = bheap![1, 2, 3, 4, 5,];
        let bheap2_test = {
            let mut bh = BinaryHeap::new();
            bh.push(1);
            bh.push(2);
            bh.push(3);
            bh.push(4);
            bh.push(5);
            bh
        };

        for (test, bh_i) in bheap2_test.iter().zip(bheap2.iter()) {
            assert_eq!(test, bh_i);
        }
    }

    #[test]
    fn trailing_all() {
        hmap! {"a" => 1,};
        hset! {1,};
        btmap! {"a" => 1,};
        btset! {1,};
        deque![1,];
        lkl![1,];
        rlkl![1,];
        bheap![1,];
    }
}
