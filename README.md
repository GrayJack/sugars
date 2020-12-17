# Sugars - Nice Rust macros for better writing.
[![Crates.io](https://img.shields.io/crates/v/sugars.svg)](https://crates.io/crates/sugars)
[![Documentation](https://docs.rs/sugars/badge.svg)](https://docs.rs/sugars)
[![License](https://img.shields.io/github/license/GrayJack/sugars.svg)](./LICENSE)
[![Github Actions](https://github.com/GrayJack/sugars/workflows/Build/badge.svg)](https://github.com/GrayJack/sugars/actions)
[![Build Status](https://travis-ci.com/GrayJack/sugars.svg?branch=master)](https://travis-ci.com/GrayJack/sugars)
[![Hits-of-Code](https://hitsofcode.com/github/GrayJack/sugars)](https://hitsofcode.com/view/github/GrayJack/sugars)

This crate provides a collection of useful macros to make tasks easier.

## What it has to offer?
 * **Macros for `std::collections`:**
    * [**deque**]: Create **`VecDeque`** from list of elements.
    * [**hset**]: Create **`HashSet`** “ .
    * [**btset**]: Create **`BTreeSet`** “ .
    * [**bheap**]: Create **`BinaryHeap`** “ .
    * [**hmap**]: Create **`HashMap`** from key-value pairs.
    * [**btmap**]: Create **`BTreeMap`** “ .
    * [**lkl**]: Create **`LinkedList`** from list of elements.
    * [**rlkl**]: Create **`LinkedList`**, but reversed.
 * **Macros for `.collect()` comprehensions:**
    * [**c**]: Macro to make lazy `Iterator` collection comprehensions, others below are
      based on this one.
    * [**cbheap**]: Build **`BinaryHeap`** with collection comprehensions.
    * [**cbtmap**]: Build **`BTreeMap`** with “ .
    * [**cbtset**]: Build **`BTreeSet`** with “ .
    * [**cdeque**]: Build **`VecDeque`** with “ .
    * [**cmap**]: Build **`HashMap`** with “ .
    * [**cset**]: Build **`HashSet`** with “ .
    * [**cvec**]: Build **`Vec`** with “ .
 * **Smart Pointers:**
    * [**arc**]: Create new **`Arc`**.**¹**
    * [**boxed**]: Create new **`Box`**.**¹**
    * [**cell**]: Create new **`Cell`**.**¹**
    * [**mutex**]: Create new **`Mutex`**.**¹**
    * [**refcell**]: Create new **`RefCell`**.**¹**
    * [**rc**]: Create new **`Rc`**.**¹**
    * [**rwlock**]: Create new **`RwLock`**.**¹**
    * [**cow**]: Create new **`Cow`**.
 * **Time/Duration:**
    * [**dur**]: Creates a **`Duration`** object following a time pattern.**²**
    * [**sleep**]: Makes current thread sleep an custom time amount.**²**
    * [**time**]: Print out the time it took to execute a given expression in seconds.

 1. Returns a tuple if multiple parameters are given.
 2. Accepted time patterns are: `min`, `sec`, `nano`, `micro` and `milli`.

## Examples
### `std::collections`
Usage of **`boxed`**, same as **`arc`**, **`cell`**, **`cow`**, **`mutex`** and **`refcell`**:
```rust
assert_eq!(Box::new(10), boxed!(10));
```

Usage of **`hmap`**, same as **`btmap`**:
```rust
let mut map = HashMap::new();
map.insert("1", 1);
map.insert("2", 2);
map.insert("3", 3);

let map2 = hmap! {"1" => 1, "2" => 2, "3" => 3};

assert_eq!(map, map2);
```

Usage of **`hset`**, same as **``btset``**:
```rust
let set = hset! {1, 2, 3};

let mut set2 = HashSet::new();
set2.insert(1);
set2.insert(2);
set2.insert(3);

assert_eq!(set, set2);
```

Usage of **`deque`**, same as **`bheap`**, **`lkl`** and **`rlkl`**:
```rust
let deque = deque![1, 2, 3];

let mut deque2 = VecDeque::new();
deque2.push_back(1);
deque2.push_back(2);
deque2.push_back(3);

assert_eq!(deque2, deque);
```

### Comprenhensions
Usage of **`c!`**: It follows the syntax: `c![<expr>; <<pattern> in <iterator>, >...[, if <condition>]]`.

Note that it generates a lazy _Iterator_ that needs to be dealt with.
```rust
let vec = c![x; x in 0..10].collect::<Vec<_>>();
let set = c![i*2; &i in vec.iter()].collect::<HashSet<_>>();
// A more complex one
let vec = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect::<Vec<_>>();

// Or using type hints
let vec: Vec<_> = c![x; x in 0..10].collect();
let set: HashSet<_> = c![i*2; &i in vec.iter()].collect();
let vec: Vec<_> = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect();
```

Usage of **`cvec!`**, same as **`cdeque!`**, **`clkl!`** and **`cbheap!`**:
```rust
// Normal comprehension
cvec![x; x in 0..10];

// You can filter as well
cvec![x; x in 0..10, if x % 2 == 0];
```

Usage of **`cset`**, same as **`cbtset`**:
```rust
// Normal comprehension
cset! {x; x in 0..10};

// You can filter as well
cset! {x; x in 0..10, if x % 2 == 0};
```

Usage of **`cmap`**, same as **`cbtmap`**:
```rust
// Normal comprehension
cmap! {x => x*2; x in 1..10};

// You can filter as well
cmap! {x => x*2; x in 1..10, if x % 2 == 0};
```

### Time/Duration:
Usage of **`dur`** and **`sleep`**:
```rust
let d1 = dur!(10 sec);
let d2 = std::time::Duration::from_secs(10);

assert_eq!(d1, d2);

// Same syntax, but make the thread sleep for the given time
sleep!(10 sec)
```

Usage of **`time`**:
```rust
// Should print to stderr ≈ 2.0000 seconds
time!( sleep!(2 sec) );

// It also return the evaluated expression, like dbg! macro
let x = time!( 100 + 20 );
```

## Minimal Viable Rust Version
This software requires Rust version equal or above 1.39.0.

## LICENSE
This software is licensed under the [MIT Public License](./LICENSE).

[**deque**]: https://docs.rs/sugars/latest/sugars/macro.bheap.html
[**hset**]: https://docs.rs/sugars/latest/sugars/macro.hset.html
[**btset**]: https://docs.rs/sugars/latest/sugars/macro.btset.html
[**bheap**]: https://docs.rs/sugars/latest/sugars/macro.bheap.html
[**hmap**]: https://docs.rs/sugars/latest/sugars/macro.hmap.html
[**btmap**]: https://docs.rs/sugars/latest/sugars/macro.btmap.html
[**lkl**]: https://docs.rs/sugars/latest/sugars/macro.lkl.html
[**rlkl**]: https://docs.rs/sugars/latest/sugars/macro.rlkl.html
[**c**]: https://docs.rs/sugars/latest/sugars/macro.c.html
[**cbheap**]: https://docs.rs/sugars/latest/sugars/macro.cbheap.html
[**cbtmap**]: https://docs.rs/sugars/latest/sugars/macro.cbtmap.html
[**cbtset**]: https://docs.rs/sugars/latest/sugars/macro.cbtset.html
[**cdeque**]: https://docs.rs/sugars/latest/sugars/macro.cdeque.html
[**cmap**]: https://docs.rs/sugars/latest/sugars/macro.cmap.html
[**cset**]: https://docs.rs/sugars/latest/sugars/macro.cset.html
[**cvec**]: https://docs.rs/sugars/latest/sugars/macro.cvec.html
[**arc**]: https://docs.rs/sugars/latest/sugars/macro.arc.html
[**boxed**]: https://docs.rs/sugars/latest/sugars/macro.boxed.html
[**cell**]: https://docs.rs/sugars/latest/sugars/macro.cell.html
[**mutex**]: https://docs.rs/sugars/latest/sugars/macro.mutex.html
[**refcell**]: https://docs.rs/sugars/latest/sugars/macro.refcell.html
[**rc**]: https://docs.rs/sugars/latest/sugars/macro.rc.html
[**rwlock**]: https://docs.rs/sugars/latest/sugars/macro.rwlock.html
[**cow**]: https://docs.rs/sugars/latest/sugars/macro.cow.html
[**dur**]: https://docs.rs/sugars/latest/sugars/macro.dur.html
[**sleep**]: https://docs.rs/sugars/latest/sugars/macro.sleep.html
[**time**]: https://docs.rs/sugars/latest/sugars/macro.time.html
