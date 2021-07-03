/*!
# Write Atomic

[![Documentation](https://docs.rs/write_atomic/badge.svg)](https://docs.rs/write_atomic/)
[![crates.io](https://img.shields.io/crates/v/write_atomic.svg)](https://crates.io/crates/write_atomic)
[![Build Status](https://github.com/Blobfolio/write_atomic/workflows/Build/badge.svg)](https://github.com/Blobfolio/write_atomic/actions)

Write Atomic is a stripped-down remake of [`tempfile-fast`](https://crates.io/crates/tempfile-fast), boiling everything down to a single method: [`write_file`].

Like `tempfile-fast`, bytes will first be written to a temporary file — either `O_TMPFILE` on supporting Linux systems or via the [`tempfile`](https://crates.io/crates/tempfile) crate — then moved the final destination.

When overwriting an existing file, permissions and ownership will be preserved, otherwise the permissions and ownership will default to the same values you'd get if using [`std::fs::File::create`].

Because there is just a single [`write_file`] method, this crate is only really suitable in cases where you have the path and all the bytes you want to write ready to go. If you need more granular `Read`/`Seek`/`Write` support, use `tempfile-fast` instead.

## Examples

```no_run
// One line is all it takes:
write_atomic::write_file("/path/to/my-file.txt", b"Some data!").unwrap();
```

## Installation

Add `write_atomic` to your `dependencies` in `Cargo.toml`, like:

```text,ignore
[dependencies]
write_atomic = "0.1.*"
```

*/

#![warn(clippy::filetype_is_file)]
#![warn(clippy::integer_division)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(macro_use_extern_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

#![allow(clippy::module_name_repetitions)]



#[cfg(target_os = "linux")]      mod linux;
#[cfg(not(target_os = "linux"))] mod fallback;



#[cfg(not(target_os = "linux"))] use fallback as linux;
use rand::{
	distributions::Alphanumeric,
	Rng,
	rngs::SmallRng,
	SeedableRng,
	self,
};
use std::{
	cell::UnsafeCell,
	ffi::OsString,
	fs::{
		File,
		Metadata,
	},
	io::{
		Error,
		ErrorKind,
		Result,
		Write,
	},
	path::{
		Path,
		PathBuf,
	},
};
use tempfile::NamedTempFile;



/// # Atomic File Write!
///
/// This will write bytes atomically to the specified path, maintaining
/// permissions and ownership if it already exists, or creating it anew using
/// the same default permissions and ownership [`std::fs::File::create`] would.
///
/// Atomicity is achieved by first writing the content to a temporary location.
/// On most Linux systems, this will use `O_TMPFILE`; for other systems, the
/// [`tempfile`] crate will be used instead.
///
/// ## Examples
///
/// ```no_run
/// // It's just one line:
/// write_atomic::write_file("/path/to/my/file.txt", b"Some data!")
///     .unwrap();
/// ```
///
/// ## Errors
///
/// This will bubble up any filesystem-related errors encountered along the
/// way.
pub fn write_file<P>(src: P, data: &[u8]) -> Result<()>
where P: AsRef<Path> {
	let (src, parent) = check_path(src)?;

	// Write via O_TMPFILE if we can.
	if let Ok(file) = linux::nonexclusive_tempfile(&parent) {
		write_direct(file, &src, data)
	}
	// Otherwise fall back to the trusty `tempfile`.
	else {
		write_fallback(
			tempfile::Builder::new().tempfile_in(parent)?,
			&src,
			data,
		)
	}
}

/// # Handle Path.
///
/// This checks the path and returns it and its parent, assuming it is valid,
/// or an error if not.
fn check_path<P>(src: P) -> Result<(PathBuf, PathBuf)>
where P: AsRef<Path> {
	let src = src.as_ref();

	// The path cannot be a directory.
	if src.is_dir() {
		return Err(Error::new(ErrorKind::InvalidInput, "Path cannot be a directory."));
	}

	// We don't need to fully canonicalize the path, but if there's no stub, it
	// is assumed to be in the "current directory".
	let src: PathBuf =
		if src.is_absolute() { src.to_path_buf() }
		else {
			let mut absolute = std::env::current_dir()?;
			absolute.push(src);
			absolute
		};

	// Make sure it has a parent.
	let parent: PathBuf = src.parent()
		.map(Path::to_path_buf)
		.ok_or_else(|| Error::new(ErrorKind::NotFound, "Path must have a parent directory."))?;

	// Create the directory chain if necessary.
	std::fs::create_dir_all(&parent)?;

	// We're good to go!
	Ok((src, parent))
}

/// # Copy Metadata.
///
/// Make sure we don't lose details like permissions, ownership, etc., when
/// replacing an existing file.
fn copy_metadata(src: &Path, dst: &File) -> Result<()> {
	let metadata = match src.metadata() {
		Ok(metadata) => metadata,
		Err(ref e) if ErrorKind::NotFound == e.kind() => return Ok(()),
		Err(e) => return Err(e),
	};

	dst.set_permissions(metadata.permissions())?;

	#[cfg(unix)]
	copy_ownership(&metadata, dst)?;

	Ok(())
}

#[cfg(unix)]
/// # Copy Ownership.
///
/// On Unix systems, we need to copy ownership in addition to permissions.
fn copy_ownership(source: &Metadata, dest: &File) -> Result<()> {
	use std::os::unix::{
		fs::MetadataExt,
		io::AsRawFd,
	};

	let fd = dest.as_raw_fd();
	if 0 == unsafe { libc::fchown(fd, source.uid(), source.gid()) } { Ok(()) }
	else { Err(Error::last_os_error()) }
}

thread_local! {
	static THREAD_RNG: UnsafeCell<SmallRng> = UnsafeCell::new(SmallRng::from_entropy());
}

/// # Random Tempfile Name
///
/// This is similar to how `tempfile` handles it. The resulting name starts
/// with a dot, ends with `.tmp`, and in between there are 10 random
/// alphanumeric characters.
///
/// To avoid temporary allocations, each random character is inserted into the
/// `Ostring` one-at-a-time. It's a bit janky-looking, but gets the job done.
fn random_name() -> OsString {
	let mut buf = OsString::with_capacity(15);
	buf.push(".");
	THREAD_RNG.with(|rng| unsafe {
		(&mut *rng.get())
			.sample_iter(&Alphanumeric)
			.take(10)
			.for_each(|b| buf.push(std::str::from_utf8_unchecked(&[b])))
	});
	buf.push(".tmp");
	buf
}

/// # Touch If Needed.
///
/// This creates paths that don't already exist to set the same default
/// permissions and ownerships the standard library would.
fn touch_if(src: &Path) -> Result<bool> {
	if src.exists() { Ok(false) }
	else {
		File::create(&src)?;
		Ok(true)
	}
}

/// # Write Direct.
///
/// This is an optimized file write for modern Linux installs.
fn write_direct(mut file: File, dst: &Path, data: &[u8]) -> Result<()> {
	file.write_all(data)?;
	file.flush()?;

	let touched = touch_if(dst)?;
	match write_direct_end(&mut file, dst) {
		Ok(()) => Ok(()),
		Err(e) => {
			// If we created the file earlier, try to remove it.
			if touched { let _res = std::fs::remove_file(dst); }
			Err(e)
		}
	}
}

/// # Finish Write Direct.
fn write_direct_end(file: &mut File, dst: &Path) -> Result<()> {
	// Copy metadata.
	copy_metadata(dst, file)?;

	// If linking works right off the bat, hurray!
	if linux::link_at(file, dst).is_ok() {
		return Ok(());
	}

	// Otherwise we need a a unique location.
	let mut dst_tmp = dst.to_path_buf();
	for _ in 0..32768 {
		// Build a new file name.
		dst_tmp.set_file_name(random_name());

		match linux::link_at(file, &dst_tmp) {
			Ok(()) => return std::fs::rename(&dst_tmp, dst).map_err(|e| {
				// That didn't work; attempt cleanup.
				let _res = std::fs::remove_file(&dst_tmp);
				e
			}),
			Err(e) => {
				// Collisions just require another go; for other errors, we
				// should abort.
				if ErrorKind::AlreadyExists != e.kind() { return Err(e); }
			}
		};
	}

	// If we're here, we've failed.
	Err(Error::new(ErrorKind::Other, "Couldn't create a temporary file."))
}

/// # Write Fallback.
///
/// For systems where `O_TMPFILE` is unavailable, we can just use the
/// `tempfile` crate.
fn write_fallback(mut file: NamedTempFile, dst: &Path, data: &[u8]) -> Result<()> {
	file.write_all(data)?;
	file.flush()?;

	let touched = touch_if(dst)?;
	match write_fallback_finish(file, dst) {
		Ok(()) => Ok(()),
		Err(e) => {
			// If we created the file earlier, try to remove it.
			if touched { let _res = std::fs::remove_file(dst); }
			Err(e)
		}
	}
}

/// # Finish Write Fallback.
fn write_fallback_finish(file: NamedTempFile, dst: &Path) -> Result<()> {
	copy_metadata(dst, file.as_file())
		.and_then(|_| file.persist(dst).map(|_| ()).map_err(|e| e.error))
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_write() {
		// Hopefully sandboxes running this test can write to their own
		// temporary directory!
		let mut path = std::env::temp_dir();
		if ! path.is_dir() { return; }
		path.push(random_name());

		// Make sure the name is unique.
		while path.exists() {
			path.set_file_name(random_name());
		}

		// Now that we have a path, let's try to write to it!
		assert!(write_file(&path, b"This is the first write!").is_ok());

		// Make sure the content is written correctly.
		assert_eq!(
			std::fs::read(&path).expect("Unable to open file."),
			b"This is the first write!",
		);

		// One more time with different content.
		assert!(write_file(&path, b"This is the second write!").is_ok());

		// Make sure the content is written correctly.
		assert_eq!(
			std::fs::read(&path).expect("Unable to open file."),
			b"This is the second write!",
		);

		// Let's clean up after ourselves.
		let _res = std::fs::remove_file(path);
	}
}
