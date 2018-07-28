# Rust vs Go

WIP slides for a planned talk at a gopher meetup.

Slides are markdown rendered with Rust/[yew](https://github.com/DenisKolodin/yew),
compiled to web assembly.

## Dev

Prerequisites:

- [https://github.com/raphamorim/wasm-and-rust](https://github.com/raphamorim/wasm-and-rust)
- cargo-web

Start:

```
cargo web start
```

Navigate to `localhost:8000`. Move between slides with left/right arrows or backspace and enter.

## TODO

Must:

- Animations between slides.
- Actual slide content...

Nice to have:

- Parse md as file instead of coded string.
- Build static bundle and/or host somewhere.
