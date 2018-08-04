pub const SLIDE_MARKDOWN: & 'static str = r#"
---
# Gopher vs Ferris
***

|    |    |
|:---|---:|
| ![](strongGopher.png)| ![](strongFerris.jpg) ![](angryFerris.jpg) |

---
# Personal Motivation
***

![](blog-post.png)

|                |                 |
| --------------- | -----------------|
| ![](scala.png) | ![](gopher.png) |

---
![](rust-evangelism.png)

> "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety." - rust-lang.org

***

- guaranteed memory safety
- threads without data races
- minimal runtime
- zero-cost abstractions
- type inference
- functional features
- trait-based generics


[Organizations Running Rust in Prod](https://www.rust-lang.org/en-US/friends.html)

[Cool projects done with Rust](https://github.com/rust-unofficial/awesome-rust)


---

### Type Inference and Immutability

> "In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through." - doc.rust-lang.org/book/second-edition

***

[https://bit.ly/2KkNbeT](https://bit.ly/2KkNbeT)

---
<iframe src="https://bit.ly/2KkNbeT" />
---

### Zero Cost Abstraction

> "In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better." - Bjarne Stroustrup

***

[https://bit.ly/2AxlbF4](https://bit.ly/2AxlbF4)

---
<iframe src="https://bit.ly/2AxlbF4" />
---

### Functional Features

> "Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions." - doc.rust-lang.org/book/second-edition

***

[https://bit.ly/2n6J4Kk](https://bit.ly/2n6J4Kk)

---
<iframe src="https://bit.ly/2n6J4Kk" />
---

### Borrowing and Ownership

> "Rust enforces RAII (Resource Acquisition Is Initialization), so whenever an object goes out of scope, its destructor is called and its owned resources are freed. This behavior shields against resource leak bugs, so you'll never have to manually free memory or worry about memory leaks again!" - doc.rust-lang.org/rust-by-example

***
[https://bit.ly/2v6mu91](https://bit.ly/2v6mu91)

---
<iframe src="https://bit.ly/2v6mu91" />
---

### Thread Safety and "Fearless Concurrency"

> "By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem. We’ve nicknamed this aspect of Rust fearless concurrency." - doc.rust-lang.org/book/second-edition

// TODO CHANGE TO MAP EXAMPLES
Concurrency in Go:
[https://goplay.space/#NJH4SckyvDD](https://goplay.space/#NJH4SckyvDD)

Reproduced in Rust:
[https://bit.ly/2O2F5ty](https://bit.ly/2O2F5ty)

Actually Compiling in Rust:
[https://bit.ly/2ODYfXS](https://bit.ly/2ODYfXS)

---
<iframe src="https://goplay.space/#NJH4SckyvDD" />
---
<iframe src="https://bit.ly/2O2F5ty" />
---
<iframe src="https://bit.ly/2ODYfXS" />
---

### Go and Rust Similarities
***

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

### Rust Difference I Prefer
***

- Very low cost call to C code vs very costly overheard Cgo.
- Generally faster, more efficient, minimal runtime vs Go garbage collected, a bit bigger run time.
- Guaranteed memory safety.
- Guaranteed data-race/ thread safety.
- Opt in to shared memory, locks data not code.
- No nulls.
- Higher level abstractions without the cost...
- Functional features.

---

### Go Differences I Prefer
***

- Compile time is much faster. Vs rust... rust is really slow, especially release build. Because the compiler is doing a lot of optimisations.
- Easier to learn because clean, simple syntax. Vs a lot more features in Rust: generics, traits, lifetimes/borrow.
- Very good batteries included std lib. Vs rust... they prefer moving core libs out to crates, which I personally dislike.
- Gets out of the way, lets you get stuff done.
- Cross platform compilation is a breeze vs rust you need to setup your environments, install certain targets. Use of open source docker containers.
- Green threads and easy to use concurrency primitives out of the box (go channels, go routines). Vs rust, more verbose libraries, OS level threads by default, has message passing but not the simple blocking kind like in Go.

---

### Resources
***

- For learning Rust: [https://doc.rust-lang.org/book/second-edition](https://doc.rust-lang.org/book/second-edition)

- For Playing with Rust: [https://play.rust-lang.org/](https://play.rust-lang.org/)

- These ugly slides: [https://github.com/jackyzhen/rust-vs-go-slides](https://github.com/jackyzhen/rust-vs-go-slides)

- Good rust vs go code comparison: [https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9](https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9)

- Good illustration of thread safety in rust vs go:  [https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29)

- Another versus article: [https://matthias-endler.de/2017/go-vs-rust/](https://matthias-endler.de/2017/go-vs-rust/)

"#;
