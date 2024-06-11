# Steps to reproduce

```bash
git clone git@github.com:Mcdostone/go-pdk-error-handling.git
cd go-pdk-error-handling

cp plugin/scenario1 plugin/main.go 
tinygo build -o plugin.wasm -target wasi plugin/main.go
cargo run
# the returned code is 0, rust code doesn't panic. It is the expected behavior.


cp plugin/scenario2 plugin/main.go 
cd plugin && tinygo build -o ../plugin.wasm -target wasi main.go
cd ..
cargo run
# the Rust code panics even though the returned code is 0


cp plugin/scenario3 plugin/main.go 
tinygo build -o plugin.wasm -target wasi plugin/main.go
cargo run
# the returned code is 500, rust code should panic but it doesn't
```