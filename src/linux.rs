/*!
# Write Atomic - Linux Optimizations

This module has been adapted from [`tempfile-fast`](https://github.com/FauxFaux/tempfile-fast-rs/blob/7cd84b28029250c96970265141448a04cafb8f60/src/linux.rs).
*/

use libc::{
	AT_FDCWD,
	AT_SYMLINK_FOLLOW,
	O_CLOEXEC,
	O_RDWR,
	O_TMPFILE,
};
use std::{
	ffi::CString,
	fs::File,
	io::{
		Error,
		ErrorKind,
		Result,
	},
	os::unix::{
		ffi::OsStrExt,
		io::{
			AsRawFd,
			FromRawFd,
		},
	},
	path::Path,
};



#[allow(unsafe_code)]
/// # Create Non-exclusive Tempfile.
pub(super) fn nonexclusive_tempfile<P>(dir: P) -> Result<File>
where P: AsRef<Path> {
	let path = cstr(dir)?;
	match unsafe { libc::open64(path.as_ptr(), O_CLOEXEC | O_TMPFILE | O_RDWR, 0o600) } {
		-1 => Err(ErrorKind::InvalidInput.into()),
		fd => Ok(unsafe { FromRawFd::from_raw_fd(fd) }),
	}
}

#[allow(unsafe_code)]
/// # Link At.
///
/// Attempt to update the file system link for a given file.
pub(super) fn link_at<P>(what: &File, dst: P) -> Result<()>
where P: AsRef<Path> {
	let old_path: CString = CString::new(format!("/proc/self/fd/{}", what.as_raw_fd()))?;
	let new_path = cstr(dst)?;

	unsafe {
		if 0 == libc::linkat(
			AT_FDCWD,
			old_path.as_ptr().cast(),
			AT_FDCWD,
			new_path.as_ptr().cast(),
			AT_SYMLINK_FOLLOW,
		) { Ok(()) }
		else { Err(Error::last_os_error()) }
	}
}



/// # `Path` to `CString`
fn cstr<P>(path: P) -> Result<CString>
where P: AsRef<Path> {
	CString::new(path.as_ref().as_os_str().as_bytes())
		.map_err(|_| Error::new(ErrorKind::InvalidInput, "Path contains a null."))
}
