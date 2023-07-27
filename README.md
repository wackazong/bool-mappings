# bool-mappings

Useful extensions to convert `bool` to other Rust types.

At the moment there are two extensions:

- `.true_or()`
- `.false_or()`

## Examples

```rust
use bool_mappings::BoolMappings;

struct MyError;

// Turn a bool into a Result
fn some_fn() -> Result<(), MyError> {
    true.true_or(MyError)
}

fn some_other_fn() -> Result<(), MyError> {
    true.false_or(MyError)
}
```

License: MIT
