# RUST ClIPPY(Code linter)
- Installation `rustup component add clippy`


# CARGO WATCH(Auto Compiling, and code monitoring)
- Installation `cargo install cargo-watch`
- Run `cargo-watch -qc -x run -x clippy`

# Rust Lifetime rules
1. Compiler assigns lifetime to every parameter that's a referennce
2. Single input lifetime is assigned to all outputs
3. If `&self` or `&mut Self` in parameters, lifetime of self is assigned to output

# Rust Read More .. 
- Vectors: `https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice`
- Harshmaps: `https://doc.rust-lang.org/std/collections/struct.HashMap.html`
- Iterartors: `https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html`
- Optionals: `https://doc.rust-lang.org/std/option/`
- Lifetimes: `https://doc.rust-lang.org/nomicon/lifetimes.html`
- Traits: `https://doc.rust-lang.org/rust-by-example/trait.html`
- Pointers: `https://doc.rust-lang.org/std/primitive.pointer.html`, `https://doc.rust-lang.org/book/ch15-04-rc.html`