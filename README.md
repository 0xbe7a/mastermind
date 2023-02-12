<!-- LTeX: language=en-US -->
# A Mastermind solver for optimal worst-case guesses
This is a solver for the [Mastermind board
game](https://en.wikipedia.org/wiki/Mastermind_(board_game)) which finds guesses that minimize the maximum number of possibilities that remain after receiving the corresponding hint

<img 
width=400
src="https://upload.wikimedia.org/wikipedia/commons/2/2d/Mastermind.jpg" />

[*Image source*](https://commons.wikimedia.org/wiki/File:Mastermind.jpg)

It is implemented using the new [std::simd](https://doc.rust-lang.org/std/simd/index.html) SIMD Abstraction from Rust to target NEON, SSE2, AVX2 and WebAssembly SIMD from a shared implementation

Additionally, it uses [rayon](https://github.com/rayon-rs/rayon) to enable multithreading

It finds optimal worst-case guesses [^1] from all non-equivalent [^2] guesses that minimize the maximum number of remaining possibilities

The core solver is implemented in [libmastermind](https://github.com/0xbe7a/mastermind/blob/main/libmastermind/src/positions.rs) alongside a CLI-Solver and a Web frontend 

# Play
[https://0xbe7a.github.io/mastermind/](https://0xbe7a.github.io/mastermind/)

_NOTE: No fallback for browsers without WASM and WebAssembly SIMD has been provided (Not Safari and Firefox is currently flaky)_

## Building the Application
1. Run ```nix develop``` on Nix (or)
   1. install rust-nightly with the ```wasm32-unknown-unknown``` target and ```rust-src``` component
   2. install wasm-pack
   3. install npm
   4. target the native CPU Features using ```export RUSTFLAGS="-C target-cpu=native"```
2. To build the CLI-Solver use ```cargo build --release```
3. To build the WebApp
   1. Go to "web/www"
   2. use ```npm install```
   3. build the application with ```npm run build```

## References:
[^1]: Knuth, D. E. (1976). The computer as master mind. Journal of Recreational Mathematics, 9(1), 1-6.

[^2]: Ville, G. (2013). An optimal mastermind (4, 7) strategy and more results in the expected case. arXiv preprint arXiv:1305.1010.
