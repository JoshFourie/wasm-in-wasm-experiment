# Wasm-in-Wasm with Wasmer

I want to prove that a `wasm32-unknown-unknown` target can run inside of a `wasm32-unknown-unknown` target by modifying the `wasmer:2.0.0 library`. 

Requires `rust`, `make` and `npm`.

# Directory
```bash
alpha/src/  # wasm32-unknown-unknown
└── lib.rs  # wasmer counter example, compiles a string so that module size is greater than 4kb

artifact-registry/src/
├── main.rs  # serves fetch requests from the resources folder
└── resources
    └── # compiled alpha.wasm module lives here

runtime/src/  # wasm32-unknown-unknown
├── fetch.rs  # fetches the alpha wasm32-unknown-unknown from localhost:7878
├── lib.rs  # sets up imports and interacts with the fetched wasm32-unknown-unknown target
└── utils.rs  # log and set_panic macros
```
# How to run
```bash
make build  # compiles the alpha, artifact
make artifact-registry-serve  # serves the alpha.wasm file on localhost:7878
make runtime-serve  # compiles a wasm32-unknown-unknown target into an app served on localhost:8080
```

On `localhost:8080` the developer console will have printed some values:
- `1` is the incremented counter from the `wasmer` example
- `[]` is the blank results slice returned by the `alpha.wasm` module - this is a bug because the exported `app` function should return an integer

You can do this with `js-sys` but the `wasmer` `import_object` wrapping simplifies the management of `wasm` binary imports.
