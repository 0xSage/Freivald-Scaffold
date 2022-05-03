# Freivald-Rust

An unsafe, Rust implementation of Freivald's algorithm on matrix multiplication verification.
It does not panic at overflows, and merely continues execution.

The point is to illustrate Freivalds’ algorithm does save runtime compared to the best known deterministic algorithm.

### Get Started
Implement the `unimplemented!` block, then run:

```rust
cargo test
cargo bench
```
