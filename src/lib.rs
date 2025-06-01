/*!
# Write Atomic

[![docs.rs](https://img.shields.io/docsrs/write_atomic.svg?style=flat-square&label=docs.rs)](https://docs.rs/write_atomic/)
[![changelog](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/write_atomic/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/write_atomic.svg?style=flat-square&label=crates.io)](https://crates.io/crates/write_atomic)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/write_atomic/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/write_atomic/actions)
[![deps.rs](https://deps.rs/crate/write_atomic/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/write_atomic/)<br>
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



use filetime::FileTime;
use std::{
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

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

// Re-export both dependencies.
pub use filetime;
pub use tempfile;



/// # Atomic File Copy!
///
/// Copy the contents — and permissions, ownership, and access/modification
/// times — of one file to another, atomically.
///
/// Similar to [`write_file`], this method first copies everything over to a
/// temporary file before moving it into place.
///
/// ## Examples
///
/// ```no_run
/// // It's just one line:
/// match write_atomic::copy_file("/some/source.jpg", "/some/copy.jpg") {
///     // The file was copied!
///     Ok(()) => {},
///
///     // There was an std::io::Error.
///     Err(e) => panic!("{e}"),
/// };
/// ```
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
	let meta = std::fs::metadata(src)?;
	copy_metadata(&meta, file.as_file(), true)?;
	write_finish(file, &dst)
}

/// # Atomic File Write!
///
/// Write content to a file, atomically.
///
/// Under the hood, this method creates a temporary file to hold all the
/// changes, then moves that file into place once everything is good to go.
///
/// If a file already exists at the destination path, this method will (try
/// to) preserve its permissions and ownership.
///
/// If not, it will simply create it.
///
/// Unlike [`File::create`](std::fs::File::create), this method will also
/// attempt to create any missing parent directories.
///
/// ## Examples
///
/// ```no_run
/// // It's just one line:
/// match write_atomic::write_file("/path/to/my/file.txt", b"Some data!") {
///     // The file was saved!
///     Ok(()) => {},
///
///     // There was an std::io::Error.
///     Err(e) => panic!("{e}"),
/// };
/// ```
///
/// ## Errors
///
/// This will bubble up any filesystem-related errors encountered along the
/// way.
pub fn write_file<P>(dst: P, data: &[u8]) -> Result<()>
where P: AsRef<Path> {
	let (dst, parent) = check_path(dst)?;

	let mut file = tempfile::Builder::new().tempfile_in(parent)?;
	file.write_all(data)?;
	file.flush()?;

	try_copy_metadata(&dst, file.as_file())?;
	write_finish(file, &dst)
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
fn copy_metadata(src: &Metadata, dst: &File, times: bool) -> Result<()> {
	// Copy permissions.
	dst.set_permissions(src.permissions())?;

	#[cfg(unix)]
	// Copy ownership.
	std::os::unix::fs::fchown(dst, Some(src.uid()), Some(src.gid()))?;

	// Copy file times too?
	if times {
		let atime = FileTime::from_last_access_time(src);
		let mtime = FileTime::from_last_modification_time(src);
		let _res = filetime::set_file_handle_times(dst, Some(atime), Some(mtime));
	}

	Ok(())
}

/// # Try Copy Metadata.
///
/// For `write_file` operations, there isn't necessarily an existing file to
/// copy permissions from.
///
/// This method will (temporarily) create one if missing so that the default
/// file permissions can at least be synced.
fn try_copy_metadata(src: &Path, dst: &File) -> Result<()> {
	match std::fs::metadata(src) {
		// We have a source! Copy the metadata as normal!
		Ok(meta) => copy_metadata(&meta, dst, false),

		// The file doesn't exist; let's (briefly) create it and sync the
		// permissions.
		Err(ref e) if ErrorKind::NotFound == e.kind() => {
			let mut res = Ok(());

			// Try to create it.
			if File::create(src).is_ok() {
				// Grab the permissions.
				if let Ok(perms) = std::fs::metadata(src).map(|m| m.permissions()) {
					res = dst.set_permissions(perms);
				}

				// Clean up.
				let _res = std::fs::remove_file(src);
			}

			res
		},

		// All other errors bubble.
		Err(e) => Err(e),
	}
}

/// # Finish Write.
///
/// Persist the temporary file.
fn write_finish(file: NamedTempFile, dst: &Path) -> Result<()> {
	file.persist(dst).map(|_| ()).map_err(|e| e.error)
}



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(unix)]
	/// # Get User/Group IDs.
	fn user_group(meta: &Metadata) -> (u32, u32) {
		use std::os::unix::fs::MetadataExt;
		(meta.uid(), meta.gid())
	}

	#[test]
	fn test_file_times() {
		let mut dst = std::env::temp_dir();
		if ! dst.is_dir() { return; }
		dst.push("LICENSE-copy.txt");

		// Pull the source's details.
		let src = std::fs::canonicalize("./LICENSE")
			.expect("Missing LICENSE file?");
		let meta1 = std::fs::metadata(&src)
			.expect("Unable to read LICENSE metadata.");

		// Copy it and pull the destination's details.
		assert!(copy_file(&src, &dst).is_ok());
		let meta2 = std::fs::metadata(&dst)
			.expect("Unable to read LICENSE-copy.txt metadata.");

		// Check sameness!
		assert_eq!(
			meta1.permissions(),
			meta2.permissions(),
			"Copied permissions not equal.",
		);

		#[cfg(unix)]
		assert_eq!(
			user_group(&meta1),
			user_group(&meta2),
			"Copied ownership not equal.",
		);

		assert_eq!(
			FileTime::from_last_modification_time(&meta1),
			FileTime::from_last_modification_time(&meta2),
			"Copied mtimes not equal.",
		);

		// Let's rewrite to the same destination and re-verify the
		// details. (`write_file` only syncs permissions if overwriting.)
		write_file(&dst, b"Testing a rewrite!").expect("Write failed.");
		let meta2 = std::fs::metadata(&dst)
			.expect("Unable to read LICENSE-copy.txt metadata.");

		// Make sure we're reading something new. Haha.
		assert_eq!(meta2.len(), 18, "Unexpected file length.");

		// Check sameness!
		assert_eq!(
			meta1.permissions(),
			meta2.permissions(),
			"Copied permissions not equal.",
		);

		#[cfg(unix)]
		assert_eq!(
			user_group(&meta1),
			user_group(&meta2),
			"Copied ownership not equal.",
		);

		// This time around the times should be different!
		assert_ne!(
			FileTime::from_last_modification_time(&meta1),
			FileTime::from_last_modification_time(&meta2),
			"Mtimes shouldn't match anymore!",
		);

		// Remove the copy.
		let _res = std::fs::remove_file(dst);
	}

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
