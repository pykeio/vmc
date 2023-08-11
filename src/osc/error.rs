use std::{error::Error, fmt, string::FromUtf8Error};

use nom::error::{ErrorKind, FromExternalError, ParseError};

/// Represents errors returned by `decode` or `encode`.
#[derive(Debug)]
pub enum OSCError {
	StringError(FromUtf8Error),
	ReadError(ErrorKind),
	BadChar(char),
	BadPacket(&'static str),
	BadMessage(&'static str),
	BadString(&'static str),
	BadArg(String),
	BadBundle(String),
	BadAddressPattern(String),
	BadAddress(String),
	RegexError(String),
	Unimplemented
}

impl fmt::Display for OSCError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			OSCError::StringError(err) => write!(f, "reading OSC string as utf-8: {}", err),
			OSCError::ReadError(kind) => write!(f, "error reading from buffer: {:?}", kind),
			OSCError::BadChar(char) => write!(f, "parser error at char: {:?}", char),
			OSCError::BadPacket(msg) => write!(f, "bad OSC packet: {}", msg),
			OSCError::BadMessage(msg) => write!(f, "bad OSC message: {}", msg),
			OSCError::BadString(msg) => write!(f, "bad OSC string: {}", msg),
			OSCError::BadArg(msg) => write!(f, "bad OSC argument: {}", msg),
			OSCError::BadBundle(msg) => write!(f, "bad OSC bundle: {}", msg),
			OSCError::BadAddressPattern(msg) => write!(f, "bad OSC address pattern: {}", msg),
			OSCError::BadAddress(msg) => write!(f, "bad OSC address: {}", msg),
			OSCError::RegexError(msg) => write!(f, "OSC address pattern regex error: {}", msg),
			OSCError::Unimplemented => write!(f, "unimplemented")
		}
	}
}

impl<I> ParseError<I> for OSCError {
	fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
		Self::ReadError(kind)
	}
	fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
		other
	}

	fn from_char(_input: I, c: char) -> Self {
		Self::BadChar(c)
	}

	fn or(self, _other: Self) -> Self {
		self
	}
}

impl<I> FromExternalError<I, OSCError> for OSCError {
	fn from_external_error(_input: I, _kind: ErrorKind, e: OSCError) -> Self {
		e
	}
}

impl Error for OSCError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			OSCError::StringError(ref err) => Some(err),
			_ => None
		}
	}
}

pub type OSCResult<T> = Result<T, OSCError>;
