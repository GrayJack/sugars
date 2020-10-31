# Sugars - Nice Rust macros for better writing
[![Documentation](https://docs.rs/sugars/badge.svg)](https://docs.rs/sugars)
[![Github Actions](https://github.com/GrayJack/sugars/workflows/Build/badge.svg)](https://github.com/GrayJack/sugars/actions)
[![Build Status](https://travis-ci.com/GrayJack/sugars.svg?branch=master)](https://travis-ci.com/GrayJack/sugars)
[![License](https://img.shields.io/github/license/GrayJack/sugars.svg)](./LICENSE)
[![Hits-of-Code](https://hitsofcode.com/github/GrayJack/sugars)](https://hitsofcode.com/view/github/GrayJack/sugars)

This crate provides a collection of macros to make some tasks easier to use on Rust ecosystem.

## What it has to offer?
 * **Macros for `std::collections`:**
    * [**deque**]: Create a **`VecDeque`** from list of elements.
    * [**hset**]: Create a **`HashSet`** “ .
    * [**btset**]: Create a **`BTreeSet`** “ .
    * [**bheap**]: Create a **`BinaryHeap`** “ .
    * [**hmap**]: Create a **`HashMap`** from key-value pairs.
    * [**btmap**]: Create a **`BTreeMap`** “ .
    * [**lkl**]: Create a **`LinkedList`** from list of elements.
    * [**flkl**]: Create a **`LinkedList`**, but reversed.
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

 1. Also can return a tuple if is given more than one parameter
 2. A time pattern can be: mim, sec, nano, micro, milli

## Examples
### std::collections
Usage of **`boxed`**, similar to **`arc`**, **`cell`**, **`cow`**, **`mutex`** and **`refcell`**:
```rust
assert_eq!(Box::new(10), boxed!(10));
```

Usage of **`hmap`**, similar to **`btmap`**:
```rust
let mut map = HashMap::new();
map.insert("1", 1);
map.insert("2", 2);

let map2 = hmap! {"1" => 1, "2" => 2};

assert_eq!(map, map2);
```

Usage of **`hset`**, similar to **`bheap`**, **``btset``**, **`deque`**, **`lkl`** and **`flkl`**:
```rust
let mut set = HashSet::new();
set.insert(1);
set.insert(2);

let set2 = hset! {1, 2};

assert_eq!(set, set2);
```

### Comprenhensions
**`c`**: Notice that it generates a lazy Iterator, so the user has to deal with that

This has the following syntax: `c![<expr>; <<pattern> in <iterator>, >...[, if <condition>]]`
```rust
let vec = c![x; x in 0..10].collect::<Vec<_>>();
let set = c![i*2; &i in vec.iter()].collect::<HashSet<_>>();
let vec2 = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 != 0].collect::<Vec<_>>();
```

Usage of **`cvec`**, similar to **`cvec`**, **`cdeque`**, **`clkl`** and **`cbheap`**:
```rust
// Normal comprehension
cvec![x; x in 0..10];

// You can filter as well
cvec![x; x in 0..10, if x % 2 == 0];
```

Usage of **`cset`**, similar to **`cbtset`**:
```rust
// Normal comprehension
cset! {x; x in 0..10};

// You can filter as well
cset! {x; x in 0..10, if x % 2 == 0};
```


Usage of **`cmap`**, similar to **`cbtmap`**:
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
[**flkl**]: https://docs.rs/sugars/latest/sugars/macro.flkl.html
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
