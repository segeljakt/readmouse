# readmouse

A very small library for reading the mouse location on macOS.

# Example usage

Continuously print the mouse location:

```rust
use readmouse::Mouse;

fn main() {
  loop {
    println!("Mouse (x, y) location: {:?}", Mouse::location());
  }
}
```

# Related

[readkey](https://crates.io/crates/readkey) - Find out if a key is currently pressed on macOS.
