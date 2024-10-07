#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]
//! # Unwrap Todo
//!
//! Provides `Option::todo` and `Result::todo` methods by providing an [`UnwrapTodo`] extension trait.
//!
//! # Usage
//!
//! ```ignore
//! // Make sure you import the trait. Otherwise, these functions will not be available.
//! use unwrap_todo::UnwrapTodo;
//!
//! // handle this file not being here/valid later. I'm just prototyping!
//! let file_content = std::fs::read("greeting.txt").todo();
//! let as_string = String::from_utf8(file_content).todo();
//!
//! assert_eq!(as_string, "hey!")
//! ```

use core::fmt::Debug;

/// Provide the `todo` method on [`Option`] and [`Result`] types.
///
/// # Usage
///
/// Use the `.todo()` function like you would use [`Result::unwrap`] or [`Option::unwrap`].
/// This, however, will panic with the error message that handling is not yet implemented.
///
/// You should use this function in temporary or prototype code, allowing you to remember
/// where error handling needs to be implemented.
pub trait UnwrapTodo {
	/// What `.todo()` should return.
	type Target;

	/// Unwrap the [`UnwrapTodo::Target`] value out of this type, panicking on other cases
	/// with a "not yet implemented" message.
	fn todo(self) -> Self::Target;
}

impl<T> UnwrapTodo for Option<T> {
	type Target = T;

	/// Returns the contained [`Some`] value, consuming the self value.
	///
	/// This will panic on [`None`], with a message indicating that none is not
	/// yet handled.
	///
	/// # Panics
	///
	/// Panics if the value is [`None`].
	///
	/// # Usage
	///
	/// Use [`Option::todo`] like you would use [`Option::unwrap`], but in scenarios
	/// where panicking on None is just a temporary solution, that should be
	/// handled properly later.
	///
	/// # Examples
	///
	/// ```ignore
	/// use unwrap_todo::UnwrapTodo;
	///
	/// let path = std::path::Path::new("path/to/file.txt");
	///
	/// // handle no file extension later
	/// let extension = path.extension().todo();
	///
	/// // handle non-utf8 file extension later
	/// let extension_str = extension.to_str().todo();
	///
	/// assert_eq!(extension_str, "txt")
	/// ```
	#[inline]
	#[track_caller]
	fn todo(self) -> Self::Target {
		match self {
			Some(t) => t,
			None => unwrap_none(),
		}
	}
}

#[cold]
#[inline(never)]
#[track_caller]
fn unwrap_none() -> ! {
	panic!("None handling not yet implemented")
}

impl<T, E> UnwrapTodo for Result<T, E>
where
	E: Debug,
{
	type Target = T;

	/// Returns the contained [`Ok`] value, consuming the self value.
	///
	/// This will panic on error, with a message indicating that error handling is not
	/// yet implemented.
	///
	/// # Panics
	///
	/// Panics if the value is an [`Err`].
	///
	/// # Usage
	///
	/// Use [`Result::todo`] like you would use [`Result::unwrap`], but in scenarios
	/// where panicking on an error is just a temporary solution, that should be
	/// handled properly later.
	///
	/// # Examples
	///
	/// ```ignore
	/// # mod fs {
	/// #   pub fn read(str: &str) -> Result<Vec<u8>, std::io::Error> {
	/// #      Ok(vec![b'h', b'e', b'y', b'!'])
	/// #   }
	/// # }
	/// use unwrap_todo::UnwrapTodo;
	///
	/// // handle this file not being here later. I'm just prototyping!
	/// let file_content = fs::read("greeting.txt").todo();
	/// let as_string = String::from_utf8(file_content).todo();
	///
	/// assert_eq!(as_string, "hey!")
	/// ```
	#[inline]
	#[track_caller]
	fn todo(self) -> Self::Target {
		match self {
			Ok(t) => t,
			Err(e) => unwrap_err(&e),
		}
	}
}

#[cold]
#[inline(never)]
#[track_caller]
fn unwrap_err(e: &dyn Debug) -> ! {
	panic!("Err handling not yet implemented: {e:?}")
}
