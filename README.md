# Write Atomic

[![docs.rs](https://img.shields.io/docsrs/write_atomic.svg?style=flat-square&label=docs.rs)](https://docs.rs/write_atomic/)
[![changelog](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/write_atomic/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=crates.io)](https://crates.io/crates/write_atomic)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/write_atomic/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/write_atomic/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/write_atomic/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/write_atomic)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/write_atomic/issues)

Write Atomic was originally a stripped-down remake of [`tempfile-fast`](https://crates.io/crates/tempfile-fast), but with the `3.4.0` release of [`tempfile`](https://crates.io/crates/tempfile), it has largely been mooted.

(`tempfile` now supports Linux optimizations like `O_TMPFILE` natively.)

That said, one might still enjoy the ergonomic single-shot nature of Write Atomic's `write_file` and `copy_file` methods, as well as their permission/ownership-syncing behaviors, and so it lives on!



## Examples

```rust
// One line is all it takes:
write_atomic::write_file("/path/to/my-file.txt", b"Some data!").unwrap();
```



## Installation

Add `write_atomic` to your `dependencies` in `Cargo.toml`, like:

```
[dependencies]
write_atomic = "0.5.*"
```



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2023 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

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
