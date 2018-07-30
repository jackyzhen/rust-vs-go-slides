# Rust vs Go

WIP slides for a planned talk at a gopher meetup.

Slides are markdown rendered with Rust/[yew](https://github.com/DenisKolodin/yew),
compiled to web assembly.

Slides are separated by markdown line break `---`.
Navigation by arrow keys and/or backspace and enter.

## Dev

Prerequisites:

- [https://github.com/raphamorim/wasm-and-rust](https://github.com/raphamorim/wasm-and-rust)
- cargo-web

Start:

```
cargo web start
```

Open `localhost:8000` in browser. Move between slides with left/right arrows or backspace and enter.

## TODO

Must:

- ~~Animations between slides.~~
- Actual slide content...

Nice to have:

- Parse md as file instead of coded string.
- Build static bundle and/or host somewhere.
