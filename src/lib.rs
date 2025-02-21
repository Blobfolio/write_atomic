/*!
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

That said, one might still enjoy the ergonomic single-shot nature of Write Atomic's [`write_file`] and [`copy_file`] methods, as well as their permission/ownership-syncing behaviors, and so it lives on!

## Examples

```no_run
// One line is all it takes:
write_atomic::write_file("/path/to/my-file.txt", b"Some data!").unwrap();
```
*/

#![forbid(unsafe_code)]

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]



use std::{
	fs::File,
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



/// # Atomic File Copy!
///
/// This will copy the contents of one file to another, atomically.
///
/// Under the hood, this uses [`std::fs::copy`] to copy the file to a temporary
/// location. It then syncs the file permissions — and on Unix, the owner/group
/// — before moving it to the final destination.
///
/// See [`write_file`] for more details about atomicity.
///
/// ## Errors
///
/// This will bubble up any filesystem-related errors encountered along the
/// way.
pub fn copy_file<P>(src: P, dst: P) -> Result<()>
where P: AsRef<Path> {
	let src = src.as_ref();
	let (dst, parent) = check_path(dst)?;

	let file = tempfile::Builder::new().tempfile_in(parent)?;
	std::fs::copy(src, &file)?;

	let touched = touch_if(&dst)?;
	if let Err(e) = write_finish(file, &dst) {
		// If we created the file earlier, try to remove it.
		if touched { let _res = std::fs::remove_file(dst); }
		Err(e)
	}
	else { Ok(()) }
}

/// # Atomic File Write!
///
/// This will write bytes atomically to the specified path, maintaining
/// permissions and ownership if it already exists, or creating it anew using
/// the same default permissions and ownership [`std::fs::File::create`] would.
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
	let (dst, parent) = check_path(src)?;

	let mut file = tempfile::Builder::new().tempfile_in(parent)?;
	file.write_all(data)?;
	file.flush()?;

	let touched = touch_if(&dst)?;
	if let Err(e) = write_finish(file, &dst) {
		// If we created the file earlier, try to remove it.
		if touched { let _res = std::fs::remove_file(dst); }
		Err(e)
	}
	else { Ok(()) }
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
/// Copy the owner/group details from `src` to `dst`.
fn copy_ownership(src: &std::fs::Metadata, dst: &File) -> Result<()> {
	use std::os::unix::fs::MetadataExt;
	std::os::unix::fs::fchown(dst, Some(src.uid()), Some(src.gid()))
}

/// # Touch If Needed.
///
/// This creates paths that don't already exist to set the same default
/// permissions and ownerships the standard library would.
fn touch_if(src: &Path) -> Result<bool> {
	if src.exists() { Ok(false) }
	else {
		File::create(src)?;
		Ok(true)
	}
}

/// # Finish Write.
///
/// This attempts to copy the metadata, then persist the tempfile.
fn write_finish(file: NamedTempFile, dst: &Path) -> Result<()> {
	copy_metadata(dst, file.as_file())
		.and_then(|()| file.persist(dst).map(|_| ()).map_err(|e| e.error))
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
		path.push("write-atomic-test.txt");

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

		// Test copy!
		let path2 = path.parent()
			.expect("Missing parent?!")
			.join("copy-atomic-test.txt");
		assert!(copy_file(&path, &path2).is_ok());
		assert_eq!(
			std::fs::read(&path2).expect("Unable to open file."),
			b"This is the second write!",
		);

		// Let's clean up after ourselves.
		let _res = std::fs::remove_file(path);
		let _res = std::fs::remove_file(path2);
	}
}
