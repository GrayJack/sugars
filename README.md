# Sugars - Nice Rust macros for better writing

[![Documentation](https://docs.rs/sugars/badge.svg)](https://docs.rs/sugars)
[![Build Status](https://travis-ci.com/GrayJack/sugars.svg?token=shFam3KGN8B2PbDYxY4y&branch=master)](https://travis-ci.com/GrayJack/sugars)
[![License](https://img.shields.io/github/license/GrayJack/sugars.svg)](./LICENSE)
[![Hits-of-Code](https://hitsofcode.com/github/GrayJack/sugars)](https://hitsofcode.com/view/github/GrayJack/sugars)

This crate provides a collection of macros to make some tasks easier to use
on Rust ecosystem.

## What it has to offer?
 * **boxed**: A simple macro to make a new Box value.
 * **btmap**: Create a `BTreeMap` from a list of key-value pairs
 * **btset**: Create a `BTreeSet` from a list of elements
 * **cmap**: Macro to `HashMap` collection comprehensions¹
 * **cset**: Macro to `HashSet` collection comprehensions¹
 * **cvec**: Macro to `Vec` collection comprehensions¹
 * **dur**: Creates a `Duration` object following a time pattern²
 * **hmap**: Create a `HashMap` from a list of key-value pairs
 * **hset**: Create a `HashSet` from a list of elements
 * **sleep**: Makes a thread sleep a amount following a time pattern²
 * **time**: Print out the time it took to execute a given expression in seconds

 1. The comprehension macros supports a haskell-like as well as python-like writing syntax and have the limitation of not supporting nesting
 2. A time pattern can be: mim, sec, nano, micro, milli

## Examples
**boxed**
```rust
assert_eq!(Box::new(10), boxed!(10));
```

**hmap and btmap**: Usage are the same, just change HashMap to BTreeMap and hmap! to btmap!
```rust
let mut map = HashMap::new();
map.insert("1", 1);
map.insert("2", 2);

let map2 = hmap!{"1" => 1, "2" => 2};

assert_eq!(map, map2);
```

**hset and btset**: Usage are the same, just change HashSet to BTreeSet and hset! to btset!
```rust
let mut set = HashSet::new();
map.insert(1);
map.insert(2);

let set2 = hset!{1, 2};

assert_eq!(set, set2);
```

**dur and sleep**
```rust
let d1 = dur!(10 sec);
let d2 = Duration::from_secs(10);

assert_eq!(d1, d2);

// Sleeps uses the same syntax, but makes the thread sleep for the given time
sleep!(10 sec)
```

**cvec**: Notice that `cvec` can be nested up to 3 times max
```rust
// Normal comprehension
cvec![x | x <- 0..10];

// You can filter as well
cvec![x | x <- 0..10, if x % 2 == 0];
```

**cset**: Notice that `cset` cannot be nested
```rust
// Normal comprehension
cset!{x | x <- 0..10};

// You can filter as well
cset!{x | x <- 0..10, if x % 2 == 0};
```

**cmap**: Notice that `cmap` cannot be nested
```rust
// Normal comprehension
cmap!{x => x*2 | x <= x <- 1..10};

// You can filter as well
cmap!{x => x*2 | x <= x <- 1..10, if x%2 == 0};
```

**time**
```rust
// Should print to stderr ≈ 2.0000 seconds
time!( sleep!(2 sec) );

// It also return the evaluated expression, like dbg! macro
let x = time!( 100 + 20 );
```

## LICENSE
This software is licensed under the [MIT Public License](./LICENSE).
