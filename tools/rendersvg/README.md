# rendersvg

*rendersvg* is an [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics) rendering application.

## Build

```bash
# Build with a Qt backend
cargo build --release --features="qt-backend"
# or with a cairo backend
cargo build --release --features="cairo-backend"
# or with both.
cargo build --release --features="qt-backend cairo-backend"
```

See [BUILD.adoc](../../BUILD.adoc) for details.

## Usage
```bash
cd tools/rendersvg
cargo run --release --features="qt-backend" -- in.svg out.png
# the binary could be found at resvg/target/release/rendersvg
```

## License

*rendersvg* is licensed under the [MPLv2.0](https://www.mozilla.org/en-US/MPL/).
