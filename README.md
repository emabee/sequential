[![Latest version](https://img.shields.io/crates/v/sequential.svg)](https://crates.io/crates/sequential)
[![Build](https://img.shields.io/github/actions/workflow/status/emabee/sequential/rust.yml?branch=main)](https://github.com/emabee/sequential/actions?query=workflow%3ACI)
[![Documentation](https://docs.rs/sequential/badge.svg)](https://docs.rs/sequential)
[![License](https://img.shields.io/crates/l/sequential.svg)](https://github.com/emabee/sequential)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`sequential` is a tiny library that provides with [`Sequence`](https://docs.rs/sequential/latest/sequential/struct.Sequence.html) a sequential number generator that

* produces monotonously increasing integer numbers, starting from a configurable starting point
  and with a configurable increment
* can be fast-forwarded to skip numbers, but cannot be wound back
* stops producing values when the limit of the chosen type T is reached
* optionally (with feature serde) implements serde's `Serialize` and `Deserialize` so that its state can be persisted
* works with all unsigned integers, from `u8` to `u128`, and with `usize`.
* strictly uses `checked_*` arithmetics to avoid _panic_ caused by number overflow.

## Usage

Add `sequential` to the dependencies section in your project's `Cargo.toml`

```toml
[dependencies]
sequential = "0.5"
```

## Example

```rust
let mut sequence = Sequence::<usize>::new();

assert_eq!(sequence.next(), Some(0_u8));
assert_eq!(sequence.next(), Some(1_u8));
```

## Features

### `serde` (optional)

Adds the serialization and deserialization capability.

## Dependencies

The library has no external dependency except the optional one to `serde`.
