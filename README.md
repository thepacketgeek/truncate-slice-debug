# truncate-slice-debug

A helper trait to truncate slice `Debug` output to a provided limit of items.

# Example Usage
```rust
use truncate_slice_debug::TruncateSliceDebug;

fn main() {
    let values = vec![0, 1, 2, 3, 4, 5];

    let dbg_output = format!("{:?}", values.as_slice().truncate_debug(3));
    assert_eq!(&dbg_output, "[0, 1, 2, ...(3 more)]");

    let dbg_output = format!("{:?}", values.as_slice().truncate_debug(10));
    assert_eq!(&dbg_output, "[0, 1, 2, 3, 4, 5]");
}
```

# License

`truncate-slice-debug` is both MIT and Apache License, Version 2.0 licensed, as found
in the LICENSE-MIT and LICENSE-APACHE files.