pub const SLIDE_MARKDOWN: & 'static str = r#"
---
# Gopher vs Ferris
***

|    |    |
|:---|---:|
| ![](strongGopher.png)| ![](strongFerris.jpg) ![](angryFerris.jpg) |

---
# Motivation
***

![](blog-post.png)

|                |                 |
| --------------- | -----------------|
| ![](scala.png) | ![](gopher.png) |

---
# What is Rust?

***

![](what_is_rust.png)

---
![](rust-evangelism.png)

> "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety." - rust-lang.org

***

- zero-cost abstractions
- move semantics
- guaranteed memory safety
- threads without data races
- trait-based generics
- pattern matching
- type inference
- minimal runtime
- efficient C bindings

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

Go:
[https://goplay.space/#Ysp1zPEc1If](https://goplay.space/#Ysp1zPEc1If)

Rust Generics:
[https://bit.ly/2M9m5gF](https://bit.ly/2M9m5gF)

---
<iframe src="https://goplay.space/#Ysp1zPEc1If" />
---
<iframe src="https://bit.ly/2M9m5gF" />
---

### Functional Features

> "Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions." - doc.rust-lang.org/book/second-edition

***

rust: [https://bit.ly/2n6J4Kk](https://bit.ly/2n6J4Kk)

go: [https://goplay.space/#3yYfLsZTJk3](https://goplay.space/#3yYfLsZTJk3)

---
<iframe src="https://bit.ly/2n6J4Kk" />
---
<iframe src="https://goplay.space/#3yYfLsZTJk3" />
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

Concurrent access in Go:
[https://goplay.space/#dJuUK-bZg6g](https://goplay.space/#dJuUK-bZg6g)

Reproduced in Rust:
[https://bit.ly/2vWUynv](https://bit.ly/2vWUynv)

Actually Compiling in Rust:
[https://bit.ly/2MakV4j](https://bit.ly/2MakV4j)

---
<iframe src="https://goplay.space/#dJuUK-bZg6g" />
---
<iframe src="https://bit.ly/2vWUynv" />
---
<iframe src="https://bit.ly/2MakV4j" />
---

### Errors, Pattern Match and Enum Types
***

Nils and Errors in Go:
[https://goplay.space/#nKy9J_Ap-A0](https://goplay.space/#nKy9J_Ap-A0)

Options and Result in Rust:
[https://bit.ly/2P1axKb](https://bit.ly/2P1axKb)

---
<iframe src="https://goplay.space/#nKy9J_Ap-A0" />
---
<iframe src="https://bit.ly/2P1axKb" />
---

### Interface and Traits
***

Go struct and interface:
[https://goplay.space/#rX9Dhm2HiQe](https://goplay.space/#rX9Dhm2HiQe)

Rust struct and traits:
[https://bit.ly/2KOzN32](https://bit.ly/2KOzN32)

---
<iframe src="https://goplay.space/#rX9Dhm2HiQe" />
---
<iframe src="https://bit.ly/2KOzN32" />
---

### Go and Rust Similarities
***

- Strongly typed.

- Prefer composition over inheritance.

- Errors are values.

- Lightweight, performant, cross platform, systems programming.

- Great tooling: IDE support, formatter, LSP.

- Integrated testing and documentation.

---

### Rust > Go
***

- FFI to C code.

- Generally more performant.

- Compile time memory and thread safety guarantees.

- Functional features and higher abstractions with no run time cost.

- Package management support via Cargo.

---

### Go > Rust
***

- Fast compile.

- Easy cross compilation.

- Batteries included std lib.

- Simple, clear syntax and language features.

- Learning curve and productivity.

---
### Rust Cont...

- Unsafe Rust

- Macros

- Async/await

- Trait Objects

- Modules and crates

- ...

---

### Resources
***

- For learning Rust: [https://doc.rust-lang.org/book/second-edition](https://doc.rust-lang.org/book/second-edition)

- For playing with Rust: [https://play.rust-lang.org/](https://play.rust-lang.org/)

- These ugly slides: [https://github.com/jackyzhen/rust-vs-go-slides](https://github.com/jackyzhen/rust-vs-go-slides)

- Good rust vs go code comparison: [https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9](https://codeburst.io/should-i-rust-or-should-i-go-59a298e00ea9)

- Good illustration of thread safety in rust vs go:  [https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29)

- Another versus article: [https://matthias-endler.de/2017/go-vs-rust/](https://matthias-endler.de/2017/go-vs-rust/)

"#;
