# bool-mappings

Useful extensions to convert `bool` to other Rust types.

At the moment there is just one extension: `.true_or()`.

## Example

```rust
use crate::MyError;
use bool_mappings::*;

// Turn a bool into a Result
fn some_fn() -> Result<(), MyError> {
    true.true_or(MyError)?
}
```
