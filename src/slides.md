---
# Gopher vs Ferris

|    |    |
|:---|---:|
| ![](strongGopher.png)| ![](strongFerris.jpg) ![](angryFerris.jpg) |

---
# Why?

|    |   |
|----|---|
| ![](blog-post.png) |
| ![](scala-spiral.png) | ![](gopher.png) |

---
# Rust!

> "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety." - rust-lang.org

***

- guaranteed memory safety

- threads without data races

- minimal runtime

- zero-cost abstractions

- type inference

- functional features

- trait-based generics

***

[Organizations Running Rust in Prod](https://www.rust-lang.org/en-US/friends.html)


[Cool projects done with Rust](https://github.com/rust-unofficial/awesome-rust)

---

### Type Inference and Immutability

> "In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through." - doc.rust-lang.org/book/second-edition

***

``` rs
fn main() {

    // explicit type annotation
    let a_float: f64 = 1.0;

    // type inference
    let an_integer = integer_returning_func();

    // opt in mutability
    let mut can_change = false;
    can_change = true;

    // variable shadowing
    let can_change = false;
}
```
---

### Zero Cost Abstraction

> "In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better." - Bjarne Stroustrup

***

``` rs
// Generics, the forbidden word
fn max<T: Ord + Copy>(xs: &[T]) -> T {
    let mut max = xs[0]; // bad time if my list is empty
    // range expression
    for i in 1..xs.len() {
        if xs[i] > max {
            max = xs[i];
        }
    }
    // no semicolon = expression
    max
}

fn main() {
    let xs_int = vec![3,4,1,7,8,324];
    let xs_str = vec!["aaa", "bbb", "ccc", "ddd"];
    println!("max = {}", max(&xs_int)); // max = 324
    println!("max = {}", max(&xs_str)); // max = ddd

    // std lib max function for generic vector
    println!("max = {:?}", xs_str.iter().max()); // max = Some("ddd")
}
```

---
### Functional Features

> "Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions." - doc.rust-lang.org/book/second-edition

***

``` rs
// import deps
use std::collections::BTreeSet;

// take a list of numbers, and filter, map, sort, unique, sum...
fn really_contrived_example(int_xs: Vec<isize>)-> isize {
    int_xs.iter()
        .filter(|&&x| x >= 10)
        .map(|x| x + 1)
        .collect::<BTreeSet<isize>>().iter()
        .take(3)
        .sum()
}

fn main() {
    let numbers = vec![1, 2, 10, 13, 16, 40, 50, 60];
    println!("{}", really_contrived_example(numbers)); // 42
}
```

---
### Borrowing and Ownership

> "Rust enforces RAII (Resource Acquisition Is Initialization), so whenever an object goes out of scope, its destructor is called and its owned resources are freed. This behavior shields against resource leak bugs, so you'll never have to manually free memory or worry about memory leaks again!" - doc.rust-lang.org/rust-by-example

***

``` rs
fn take_ownership_and_destroy_list(xs: Vec<isize>) {
    println!("Destroying a list {:?}", xs);
}  // 'xs' is dropped at end of function scope and the memory freed

fn main() {
    // _Stack_ allocated integer
    let x = 5;

    // *Copy* 'x' into 'y' - no resources are 'moved'.
    // Primitive types are copy by default because their size is
    // known at compile time and allocated on stack.
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y); // x is 5, and y is 5

    // 'a' is a _heap_ allocated, growable list
    let a = vec![1, 2, 3, x, y];

    // The pointer address of 'a' is copied (not the data) into 'b'.
    // Both are now pointers to the same heap allocated data, but
    // 'b' now owns it. Move 'a' into 'b'.
    let b = a;

    //println!("a contains: {:?}", a); // ERROR

    take_ownership_and_destroy_list(b);

    //println!("b contains: {}", b); // ERROR
}
```
---
## Thread Safety and "Fearless Concurrency"

---
## Go and Rust Similarities

- Strongly typed.
- Explicit error returns. No try catch and error bubbling.
    - Rust returns Result type, which is a functional way to handle it. Forces you to handle it.
    - Go returns explicit error type, but can be ignored...
- Both pretty lightweight, cross platform.
- Both prefer composition over inheritance. No "object oriented class/inheritance". Traits are almost like interfaces. Structs in both languages. You don't really create an instances of a class.
- Both "systems programming languages", lets you go low level, touch bytes, OS calls.
- Multiple return values (tuples in rust vs multiple return function in go);
- Both have great tooling: Rust vs Go fmter, syntax HL/linting support. The compiler in Rust is especially verbose and tells you exactly which bit of the line of code is wrong, why its wrong and suggests solutions.
- Easy test integration. Go can write nice tests with standard lib/tools, same in Rust. Can embed rust test code next to actual code.

---
## Rust Difference I Prefer

- Very low cost call to C code vs very costly overheard Cgo.
- Generally faster, more efficient, minimal runtime vs Go garbage collected, a bit bigger run time.
- Guaranteed memory safety.
- Guaranteed data-race/ thread safety.
- Opt in to shared memory, locks data not code.
- No nulls.
- Higher level abstractions without the cost...
- Functional features.
---
## Go Differences I Prefer

- Compile time is much faster. Vs rust... rust is really slow, especially release build. Because the compiler is doing a lot of optimisations.
- Easier to learn because clean, simple syntax. Vs a lot more features in Rust: generics, traits, lifetimes/borrow.
- Very good batteries included std lib. Vs rust... they prefer moving core libs out to crates, which I personally dislike.
- Gets out of the way, lets you get stuff done.
- Cross platform compilation is a breeze vs rust you need to setup your environments, install certain targets. Use of open source docker containers.
- Green threads and easy to use concurrency primitives out of the box (go channels, go routines). Vs rust, more verbose libraries, OS level threads by default, has message passing but not the simple blocking kind like in Go.

---
## Resources

- For learning Rust: [https://doc.rust-lang.org/book/second-edition](https://doc.rust-lang.org/book/second-edition)

- For Playing with Rust: [https://play.rust-lang.org/](https://play.rust-lang.org/)

- These ugly slides: [https://github.com/jackyzhen/rust-vs-go-slides](https://github.com/jackyzhen/rust-vs-go-slides)

- Good rust vs go code comparison: [https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9](https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9)

- Good illustration of thread safety in rust vs go:  [https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29)

- Another versus article: [https://matthias-endler.de/2017/go-vs-rust/](https://matthias-endler.de/2017/go-vs-rust/)
