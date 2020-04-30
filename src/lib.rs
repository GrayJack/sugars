/// This crate provides a collection of macros to make some tasks easier to use
/// on Rust ecosystem.
///
/// ## What it has to offer?
///  * **arc**: A simple macro to make a new `Arc` value.³
///  * **boxed**: A simple macro to make a new `Box` value.³
///  * **btmap**: Create a `BTreeMap` from a list of key-value pairs
///  * **btset**: Create a `BTreeSet` from a list of elements
///  * **cbtmap**: Macro to `BTreeMap` collection comprehensions¹
///  * **cbtset**: Macro to `BTreeSet` collection comprehensions¹
///  * **cell**: A simple macro to make a new `Cell` value.³
///  * **cmap**: Macro to `HashMap` collection comprehensions¹
///  * **cow**: A simple macro to make a new `Cow::Owned` value.
///  * **cset**: Macro to `HashSet` collection comprehensions¹
///  * **cvec**: Macro to `Vec` collection comprehensions¹
///  * **dur**: Creates a `Duration` object following a time pattern²
///  * **flkl**: Create a `LinkedList` from a list of elements, adding each element to the start of the list.
///  * **hmap**: Create a `HashMap` from a list of key-value pairs
///  * **hset**: Create a `HashSet` from a list of elements
///  * **lkl**: Create a `LinkedList` from a list of elements, adding each element to the end of the list.
///  * **mutex**: A simple macro to make a new `Mutex` value.³
///  * **refcell**: A simple macro to make a new `RefCell` value.³
///  * **rc**: A simple macro to make a new `Rc` value.³
///  * **sleep**: Makes a thread sleep a amount following a time pattern²
///  * **time**: Print out the time it took to execute a given expression in seconds
///
///  1. The comprehension macros supports a haskell-like as well as python-like writing syntax and have the limitation of not supporting nesting
///  2. A time pattern can be: mim, sec, nano, micro, milli
///  3. Also can return a tuple if is given more than one parameter
///
/// ## Examples
/// **arc**, **boxed**, **cell**, **cow**, **mutex** and **refcell**: Usage are mostly the same, just change de Types and macros
/// ```rust,ignore
/// assert_eq!(Box::new(10), boxed!(10));
/// ```
///
/// **hmap** and **btmap**: Usage are the same, just change HashMap to BTreeMap and hmap! to btmap!
/// ```rust,ignore
/// let mut map = HashMap::new();
/// map.insert("1", 1);
/// map.insert("2", 2);
///
/// let map2 = hmap!{"1" => 1, "2" => 2};
///
/// assert_eq!(map, map2);
/// ```
///
/// **hset** and **btset**: Usage are the same, just change HashSet to BTreeSet and hset! to btset!
/// ```rust,ignore
/// let mut set = HashSet::new();
/// map.insert(1);
/// map.insert(2);
///
/// let set2 = hset!{1, 2};
///
/// assert_eq!(set, set2);
/// ```
///
/// **dur** and **sleep**
/// ```rust,ignore
/// let d1 = dur!(10 sec);
/// let d2 = Duration::from_secs(10);
///
/// assert_eq!(d1, d2);
///
/// // Sleeps uses the same syntax, but makes the thread sleep for the given time
/// sleep!(10 sec)
/// ```
///
/// **cvec**: Notice that `cvec` can be nested up to 3 times max
/// ```rust,ignore
/// // Normal comprehension
/// cvec![x; x in 0..10];
///
/// // You can filter as well
/// cvec![x; x in 0..10, if x % 2 == 0];
/// ```
///
/// **cset** and **cbtset**: Notice that `cset`/`cbtset` cannot be nested. Usage are the same, just change `HashSet` to `BTreeSet` and `cset!` to `cbtset!`
/// ```rust,ignore
/// // Normal comprehension
/// cset!{x; x in 0..10};
///
/// // You can filter as well
/// cset!{x; x in 0..10, if x % 2 == 0};
/// ```
///
/// **cmap** and **cbtmap**: Notice that `cmap`/`cbtmap` cannot be nested. Usage are the same, just change `HashMap` to `BTreeMap` and `cmap!` to `cbtmap!`
/// ```rust,ignore
/// // Normal comprehension
/// cmap!{x => x*2; x in 1..10};
///
/// // You can filter as well
/// cmap!{x => x*2; x in 1..10, if x%2 == 0};
/// ```
///
/// **time**
/// ```rust,ignore
/// // Should print to stderr ≈ 2.0000 seconds
/// time!( sleep!(2 sec) );
///
/// // It also return the evaluated expression, like dbg! macro
/// let x = time!( 100 + 20 );
/// ```

mod collections;
mod comprehension;
mod hash;
mod pointer;
mod times;
