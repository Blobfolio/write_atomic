# Write Atomic

[![Documentation](https://docs.rs/write_atomic/badge.svg)](https://docs.rs/write_atomic/)
[![crates.io](https://img.shields.io/crates/v/write_atomic.svg)](https://crates.io/crates/write_atomic)
[![Build Status](https://github.com/Blobfolio/write_atomic/workflows/Build/badge.svg)](https://github.com/Blobfolio/write_atomic/actions)


Write Atomic is a stripped-down remake of [`tempfile-fast`](https://crates.io/crates/tempfile-fast), boiling everything down to a single method: [`write_file`].

Like `tempfile-fast`, bytes will first be written to a temporary file — either `O_TMPFILE` on supporting Linux systems or via the [`tempfile`](https://crates.io/crates/tempfile) crate — then moved the final destination.

When overwriting an existing file, permissions and ownership will be preserved, otherwise the permissions and ownership will default to the same values you'd get if using `std::fs::File::create`.

Because there is just a single [`write_file`] method, this crate is only really suitable in cases where you have the path and all the bytes you want to write ready to go. If you need more granular `Read`/`Seek`/`Write` support, use `tempfile-fast` instead.



## Examples

```rust
// One line is all it takes:
write_atomic::write_file("/path/to/my-file.txt", b"Some data!").unwrap();
```



## Installation

Add `write_atomic` to your `dependencies` in `Cargo.toml`, like:

```
[dependencies]
write_atomic = "0.1.*"
```



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2021 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
