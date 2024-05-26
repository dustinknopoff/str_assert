# str_assert

Ideal for figuring out what part of a large string is causing tests to fail.

Thin wrapper around the stdlib [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html) and [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html) but using [`dissimilar`](https://crates.io/crates/dissimilar) for comparison

For example

```rust
#[test]
fn has_diff() {
    str_assert_eq!("Lorem ipsum doleret", "Lorem ipsum dolert", "Error");
}
```
Will panic with the message

```
assertion failed: `(left == right)`
  diff: [
    Equal("Lorem ipsum doler"),
    Delete("e"),
    Equal("t"),
]: Error
```