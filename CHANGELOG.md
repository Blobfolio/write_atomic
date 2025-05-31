# Changelog



## [0.7.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.7.0) - TBD

### New

* Re-export `tempfile`
* Re-export `filetime`

### Changed

* Bump MSRV to `1.87`
* `copy_file` now (tries to) copy access and modification times too
* Improved documentation
* Improved test coverage
* Miscellaneous code cleanup and lints



## [0.6.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.6.0) - 2025-02-24

### Changed

* Bump MSRV to `1.85`
* Bump Rust edition to 2024



## [0.5.3](https://github.com/Blobfolio/write_atomic/releases/tag/v0.5.3) - 2025-02-20

### Changed

* Miscellaneous code cleanup and lints



## [0.5.2](https://github.com/Blobfolio/write_atomic/releases/tag/v0.5.2) - 2024-11-28

### Changed

* Bump `tempfile` to `3.14`
* Miscellaneous code cleanup and lints



## [0.5.1](https://github.com/Blobfolio/write_atomic/releases/tag/v0.5.1) - 2024-09-05

### Changed

* Miscellaneous code cleanup and lints



## [0.5.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.5.0) - 2023-10-05

### Changed

* Bump MSRV `1.73.0`
* Drop `rustix` in favor of stable `std::os::unix::fs::fchown`
* Direct library code is now 100% safe



## [0.4.1](https://github.com/Blobfolio/write_atomic/releases/tag/v0.4.1) - 2023-09-10

### Changed

* Remove unnecessary `BufWriter` wrapper



## [0.4.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.4.0) - 2023-07-24

### Changed

* Bump `tempfile` to `3.7.0`
* Bump `rustix` to `0.38`
* Bump MSRV to `1.63`



## [0.3.2](https://github.com/Blobfolio/write_atomic/releases/tag/v0.3.2) - 2023-06-01

This release improves the unit testing coverage, but has no user-facing changes.



## [0.3.1](https://github.com/Blobfolio/write_atomic/releases/tag/v0.3.1) - 2023-03-31

### Changed

* Bump `tempfile` to `3.5.0`
* Bump `rustix` to `0.37`


## [0.3.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.3.0) - 2023-03-09

### Changed

* Use `tempfile` for all temporary file writes (it now natively supports `O_TMPFILE`);
* Replace `libc::fchown` with `rustix::fs::fchown` for better parity with `tempfile`'s dependencies;
* Improve performance of `copy_file`;



## [0.2.10](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.10) - 2023-03-03

### Changed

* Bump `tempfile` to `3.4.0`



## [0.2.9](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.9) - 2023-02-13

### Changed

* Support `fastrand` up to `1.9.0`



## [0.2.8](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.8) - 2023-01-26

### Changed

* Doc changes (copyright year, etc.)



## [0.2.7](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.7) - 2022-11-03

### Changed

* Remove unneeded borrow



## [0.2.6](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.6) - 2022-09-22

### Changed

* Lower MSRV `1.56`
* Improve docs



## [0.2.5](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.5) - 2022-07-30

### Added

* `copy_file`



## [0.2.4](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.4) - 2022-07-24

### Changed

* Bump `fastrand` 1.8.0



## [0.2.3](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.3) - 2022-05-19

### Changed

* Update and lock third-party dependency versions



## [0.2.1](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.1) - 2022-01-10

### Changed

* Update dependencies.
* Replace `rand` with `fastrand`.



## [0.2.0](https://github.com/Blobfolio/write_atomic/releases/tag/v0.2.0) - 2021-10-21

### Added

* This changelog! Haha.

### Changed

* Use Rust edition 2021.
