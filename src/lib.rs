//! A collection of macros in Rust to make tasks easier and less verbose.
//!
//! ## What it has to offer?
//!  * **Macros for `std::collections`:**
//!     * [**deque**]: Create a [`VecDeque`] from list of elements.
//!     * [**hset**]: Create a [`HashSet`] “ .
//!     * [**btset**]: Create a [`BTreeSet`] “ .
//!     * [**bheap**]: Create a [`BinaryHeap`] “ .
//!     * [**hmap**]: Create a [`HashMap`] from key-value pairs.
//!     * [**btmap**]: Create a [`BTreeMap`] “ .
//!     * [**lkl**]: Create a [`LinkedList`] from list of elements.
//!     * [**rlkl**]: Create a [`LinkedList`], but reversed.
//!  * **Macros for `.collect()` comprehensions:**
//!     * [**c**]: Macro to make lazy Iterator collection comprehensions, others below are
//!       based on this one.
//!     * [**cbheap**]: Macro to [`BinaryHeap`] collection comprehensions.
//!     * [**cbtmap**]: Macro to [`BTreeMap`] “ .
//!     * [**cbtset**]: Macro to [`BTreeSet`] “ .
//!     * [**cdeque**]: Macro to [`VecDeque`] “ .
//!     * [**cmap**]: Macro to [`HashMap`] “ .
//!     * [**cset**]: Macro to [`HashSet`] “ .
//!     * [**cvec**]: Macro to [`Vec`] “ .
//!  * **Smart Pointers:**
//!     * [**arc**]: Create new [`Arc`].**¹**
//!     * [**boxed**]: Create new [`Box`].**¹**
//!     * [**cell**]: Create new [`Cell`].**¹**
//!     * [**mutex**]: Create new [`Mutex`].**¹**
//!     * [**refcell**]: Create new [`RefCell`].**¹**
//!     * [**rc**]: Create new [`Rc`].**¹**
//!     * [**rwlock**]: Create new [`RwLock`].**¹**
//!     * [**cow**]: Create new [`Cow`].
//!  * **Time/Duration:**
//!     * [**dur**]: Creates a [`Duration`] object following a time pattern.**²**
//!     * [**sleep**]: Makes current thread sleep an amount following a time pattern.**²**
//!     * [**time**]: Print out the time it took to execute a given expression in seconds.
//!
//!  1. Also can return a tuple if is given more than one parameter
//!  2. A time pattern accepts _mim_, _sec_, _nano_, _micro_ or _milli_.
//!
//! ## Examples
//! ### std::collections
//! Usage of **`boxed`**, similar to **`arc`**, **`cell`**, **`cow`**, **`mutex`** and **`refcell`**:
//! ```rust
//! use sugars::boxed;
//! assert_eq!(Box::new(10), boxed!(10));
//! ```
//!
//! Usage of **`hmap`**, similar to **`btmap`**:
//! ```rust
//! use std::collections::HashMap;
//! use sugars::hmap;
//!
//! let mut map = HashMap::new();
//! map.insert("1", 1);
//! map.insert("2", 2);
//!
//! let map2 = hmap! {"1" => 1, "2" => 2};
//!
//! assert_eq!(map, map2);
//! ```
//!
//! Usage of **`hset`**, similar to **`bheap`**, **``btset``**, **`deque`**, **`lkl`** and **`rlkl`**:
//! ```rust
//! use std::collections::HashSet;
//! use sugars::hset;
//!
//! let mut set = HashSet::new();
//! set.insert(1);
//! set.insert(2);
//!
//! let set2 = hset! {1, 2};
//!
//! assert_eq!(set, set2);
//! ```
//!
//! ### Comprenhensions
//! **`c`**: Notice that it generates a lazy Iterator, so the user has to deal with that
//!
//! This has the following syntax: `c![<expr>; <<pattern> in <iterator>, >...[, if <condition>]]`
//! ```rust
//! use std::collections::HashSet;
//! use sugars::c;
//!
//! let vec = c![x; x in 0..10].collect::<Vec<_>>();
//! let set = c![i*2; &i in vec.iter()].collect::<HashSet<_>>();
//! let vec2 = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect::<Vec<_>>();
//! ```
//!
//! Usage of **`cvec`**, similar to **`cvec`**, **`cdeque`**, **`clkl`** and **`cbheap`**:
//! ```rust
//! use sugars::cvec;
//!
//! // Normal comprehension
//! cvec![x; x in 0..10];
//!
//! // You can filter as well
//! cvec![x; x in 0..10, if x % 2 == 0];
//! ```
//!
//! Usage of **`cset`**, similar to **`cbtset`**:
//! ```rust
//! use sugars::cset;
//!
//! // Normal comprehension
//! cset! {x; x in 0..10};
//!
//! // You can filter as well
//! cset! {x; x in 0..10, if x % 2 == 0};
//! ```
//!
//! Usage of **`cmap`**, similar to **`cbtmap`**:
//! ```rust
//! use sugars::cmap;
//!
//! // Normal comprehension
//! cmap! {x => x*2; x in 1..10};
//!
//! // You can filter as well
//! cmap! {x => x*2; x in 1..10, if x % 2 == 0};
//! ```
//!
//! ### Time/Duration:
//! **time**
//!
//! Usage of **`dur`** and **`sleep`**:
//! ```rust
//! use sugars::{dur, sleep};
//!
//! let d1 = dur!(10 sec);
//! let d2 = std::time::Duration::from_secs(10);
//!
//! assert_eq!(d1, d2);
//!
//! // Same syntax, but make the thread sleep for the given time
//! sleep!(10 sec)
//! ```
//!
//! Usage of **`time`**:
//! ```rust
//! use sugars::{time, sleep};
//!
//! // Should print to stderr ≈ 2.0000 seconds
//! time!( sleep!(2 sec) );
//!
//! // It also return the evaluated expression, like dbg! macro
//! let x = time!( 100 + 20 );
//! ```
//!
//! ## Minimal Viable Rust Version
//! This software requires Rust version equal or above 1.39.0.
//!
//! ## LICENSE
//! This software is licensed under the [MIT Public License](./LICENSE).
//!
//! [**deque**]: deque
//! [**hset**]: hset
//! [**btset**]: btset
//! [**bheap**]: bheap
//! [**hmap**]: hmap
//! [**btmap**]: btmap
//! [**lkl**]: lkl
//! [**rlkl**]: rlkl
//! [**c**]: c
//! [**cbheap**]: cbheap
//! [**cbtmap**]: cbtmap
//! [**cbtset**]: cbtset
//! [**cdeque**]: cdeque
//! [**cmap**]: cmap
//! [**cset**]: cset
//! [**cvec**]: cvec
//! [**arc**]: arc
//! [**boxed**]: boxed
//! [**cell**]: cell
//! [**mutex**]: mutex
//! [**refcell**]: refcell
//! [**rc**]: rc
//! [**rwlock**]: rwlock
//! [**cow**]: cow
//! [**dur**]: dur
//! [**sleep**]: sleep
//! [**time**]: time
//!
//! [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
//! [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
//! [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
//! [`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
//! [`Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
//! [`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
//!

mod collections;
mod comprehension;
mod hash;
mod pointer;
mod times;
