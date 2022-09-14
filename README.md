# Write Atomic

[![docs.rs](https://img.shields.io/docsrs/write_atomic.svg?style=flat-square&label=docs.rs)](https://docs.rs/write_atomic/)
[![changelog](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/write_atomic/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=crates.io)](https://crates.io/crates/write_atomic)
[![ci](https://img.shields.io/github/workflow/status/Blobfolio/write_atomic/Build.svg?style=flat-square&label=ci)](https://github.com/Blobfolio/write_atomic/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/write_atomic/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/write_atomic)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/write_atomic/issues)

**ALPHA**: Note this crate is a work-in-progress and is not yet ready for production use.

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
write_atomic = "0.2.*"
```



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2022 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

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
