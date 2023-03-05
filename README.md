## Complextro
Graph complex functions either as a desktop app or inside the browser, using either OpenGL or WebGL.



## Examples

![0](./examples/0.png)

![1](./examples/1.png)

![2](./examples/2.png)

## Building
Desktop:
````bash
cargo build --release
````

&nbsp;

Web:
````bash
cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/complextro.wasm ./web/
python -m http.server --directory web
````

## License
MIT.
