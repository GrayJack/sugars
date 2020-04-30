# Changes

## Unreleased ~ master
 - Add a collection macro for `VecDeque`: `deque!`
 - Add empty pattern for collections macros already created

## 1.1.0
 - Add for all comprehensions a Implicit-`for` (or Just-`in`) version
 - Add new version of `lkl` and `flkl` that works almost like array init syntax (`[$init_elem; $n_times]`)

## 1.0.0
 - Remove nightly requirements.

## 0.5.0
 - Add comprehension macros `cbtmap` and `cbtset` mostly for completeness
 - Add macros `lkl` and `flkl` that return a `LinkedList`, the fist pushing to back and the other to the front

## 0.4.0
 - The macro `mutex` now return a tuple of `Mutex` values if 2 or more expression are passed to it
 - The macro `refcell` now return a tuple of `RefCell` values if 2 or more expression are passed to it
 - The macro `cell` now return a tuple of `Cell` values if 2 or more expression are passed to it
 - The macro `arc` now return a tuple of `Arc` values if 2 or more expression are passed to it
 - The macro `rc` now return a tuple of `Rc` values if 2 or more expression are passed to it
 - The macro `boxed` now return a tuple of `Box` values if 2 or more expression are passed to it
 - Consistency in the `cvec` macro between 2 and 3 nested versions
 - Add `cell` macro to create a smart pointer type `Cell`
 - Now `time` macro supports more than one expression and return a tuple
 - Now `time` macro prints the expression that was passed to it

## 0.3.0
 - Add a number of macros to create smart pointers: `rc!`, `refcell!`, `cow!`, `arc!`, `mutex!`
 - Move the boxed module to pointers, related to smart pointer
 - Add a new macro `hash` that gives back the hash of passed expression (of course, as long the type implements `Hash` trait)

## 0.2.0
 - Implement 3 nested `cvec`
 - Implement 2 nested `cvec`
 - Improve of `cset` macro using `Iterators` methods (doesn't bring a lot of performance improvements, but the code is cleaner)
 - Macro `time` now prints to stderr and format the time to 6 digits after the dot
 - Improve of `cmap` macro using `Iterators` methods (doesn't bring a lot of performance improvements, but the code is cleaner)
 - Improve performance of `cvec` macro using `Iterators` methods

## 0.1.0
 - Add collections macro `hmap`, `hset`, `btmap`, `btset`
 - Add times macro `time`
 - Add times macro `sleep`
 - Add times macro `dur` to create `Duration`
 - Add macro `boxed` to create `Box`
 - Add comprehension macro `cset`
 - Add comprehension macro `cmap`
 - Add comprehension macro `cvec`
