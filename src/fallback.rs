/*!
# Write Atomic - Fallback.
*/

use std::{
	fs::File,
	io::{
		Result,
		ErrorKind,
	},
	path::Path,
};

#[inline]
pub(super) fn nonexclusive_tempfile<P>(_dir: P) -> Result<File>
where P: AsRef<Path> {
	Err(ErrorKind::InvalidInput.into())
}

#[inline]
pub(super) fn link_at<P>(_what: &File, _dst: P) -> Result<()>
where P: AsRef<Path> {
	Err(ErrorKind::InvalidData.into())
}
