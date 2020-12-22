//! [![Documentation](https://docs.rs/sugars/badge.svg)](https://docs.rs/sugars)
//! [![License](https://img.shields.io/github/license/GrayJack/sugars.svg)](https://github.com/GrayJack/sugars/blob/master/LICENSE)
//!
//! # Sugars - Nice macros for better writing
//!
//! This crate provides a collection of useful macros to make tasks easier.
//!
//! ## What it has to offer?
//!  * **Macros for [`std::collections`]:**
//!     * [**deque**]: Create [`VecDeque`] from list of elements.
//!     * [**hset**]: Create [`HashSet`] “ .
//!     * [**btset**]: Create [`BTreeSet`] “ .
//!     * [**bheap**]: Create [`BinaryHeap`] “ .
//!     * [**hmap**]: Create [`HashMap`] from key-value pairs.
//!     * [**btmap**]: Create [`BTreeMap`] “ .
//!     * [**lkl**]: Create [`LinkedList`] from list of elements.
//!     * [**rlkl**]: Create [`LinkedList`], but reversed.
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
//!  1. Returns a tuple if multiple parameters are given.
//!  2. Accepted time patterns are: `min`, `sec`, `nano`, `micro` and `milli`.
//!
//! ## Examples
//! ### `std::collections`
//! Usage of **`boxed`**, same as **`arc`**, **`cell`**, **`cow`**, **`mutex`** and **`refcell`**:
//! ```rust
//! use sugars::boxed;
//! assert_eq!(Box::new(10), boxed!(10));
//! ```
//!
//! Usage of **`hmap`**, same as **`btmap`**:
//! ```rust
//! use std::collections::HashMap;
//! use sugars::hmap;
//!
//! let map = hmap! {"1" => 1, "2" => 2, "3" => 3};
//!
//! let mut map2 = HashMap::new();
//! map2.insert("1", 1);
//! map2.insert("2", 2);
//! map2.insert("3", 3);
//!
//! assert_eq!(map, map2);
//! ```
//!
//! Usage of **`hset`**, same as **``btset``**:
//! ```rust
//! use std::collections::HashSet;
//! use sugars::hset;
//!
//! let set = hset! {1, 2, 3};
//!
//! let mut set2 = HashSet::new();
//! set2.insert(1);
//! set2.insert(2);
//! set2.insert(3);
//!
//! assert_eq!(set, set2);
//! ```
//!
//! Usage of **`deque`**, same as **`bheap`**, **`lkl`** and **`rlkl`**:
//! ```rust
//! use sugars::deque;
//! use std::collections::VecDeque;
//! let deque = deque![1, 2, 3];
//!
//! let mut deque2 = VecDeque::new();
//! deque2.push_back(1);
//! deque2.push_back(2);
//! deque2.push_back(3);
//!
//! assert_eq!(deque2, deque);
//! ```
//!
//! ### Comprenhensions
//! Usage of **`c!`**: It follows the syntax: `c![<expr>; <<pattern> in <iterator>, >...[, if <condition>]]`.
//!
//! Note that it generates a lazy _Iterator_ that needs to be dealt with.
//! ```rust
//! use std::collections::HashSet;
//! use sugars::c;
//!
//! let vec = c![x; x in 0..10].collect::<Vec<_>>();
//! let set = c![i*2; &i in vec.iter()].collect::<HashSet<_>>();
//! // A more complex one
//! let vec = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect::<Vec<_>>();
//!
//! // Or using type hints
//! let vec: Vec<_> = c![x; x in 0..10].collect();
//! let set: HashSet<_> = c![i*2; &i in vec.iter()].collect();
//! let vec: Vec<_> = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect();
//! ```
//!
//! Usage of **`cvec`**, same as **`cdeque`**, **`clkl`** and **`cbheap`**:
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
//! Usage of **`cset`**, same as **`cbtset`**:
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
//! Usage of **`cmap`**, same as **`cbtmap`**:
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
//! [`BinaryHeap`]: ::std::collections::BinaryHeap
//! [`BTreeMap`]: ::std::collections::BTreeMap
//! [`BTreeSet`]: ::std::collections::BTreeSet
//! [`HashMap`]: ::std::collections::HashMap
//! [`HashSet`]: ::std::collections::HashSet
//! [`VecDeque`]: ::std::collections::VecDeque
//! [`LinkedList`]: ::std::collections::LinkedList
//! [`Arc`]: ::std::sync::Arc
//! [`Cell`]: ::std::cell::Cell
//! [`Mutex`]: ::std::sync::Mutex
//! [`Rc`]: ::std::rc::Rc
//! [`RefCell`]: ::std::cell::RefCell
//! [`RwLock`]: ::std::sync::RwLock
//! [`Duration`]: ::std::time::Duration
//! [`Cow`]: ::std::borrow::Cow

mod collections;
mod comprehension;
mod hash;
mod pointer;
mod times;
