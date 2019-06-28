# Sugars - Nice Rust macros for better writing

[![Documentation](https://docs.rs/sugars/badge.svg)](https://crates.io/crates/sugars)
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

## LICENSE
This software is licensed under the [MIT Public License](./LICENSE).
