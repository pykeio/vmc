use std::{
	error::Error,
	fmt,
	io::{self}
};

use crate::{OSCType, osc};

#[derive(Debug)]
pub enum VMCError {
	Io(io::Error),
	Osc(osc::OSCError),
	UnimplementedMessage(String, Vec<OSCType>),
	UnknownBone(String),
	UnknownBlendShape(String),
	UnknownModelState(i32),
	UnknownCalibrationState(i32),
	UnknownCalibrationMode(i32),
	UnknownTrackingState(i32)
}

impl fmt::Display for VMCError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			VMCError::Io(err) => write!(f, "socket error: {err}"),
			VMCError::Osc(err) => write!(f, "protocol error: {err}"),
			VMCError::UnimplementedMessage(addr, args) => write!(f, "handling '{addr}' not implemented (args: {args:?})"),
			VMCError::UnknownBone(bone) => write!(f, "unknown bone: {bone}"),
			VMCError::UnknownBlendShape(blend_shape) => write!(f, "unknown blend shape: {blend_shape}"),
			VMCError::UnknownModelState(state) => write!(f, "unknown model state: {state}"),
			VMCError::UnknownCalibrationState(state) => write!(f, "unknown calibration state: {state}"),
			VMCError::UnknownCalibrationMode(mode) => write!(f, "unknown calibration mode: {mode}"),
			VMCError::UnknownTrackingState(state) => write!(f, "unknown tracking state: {state}")
		}
	}
}

impl From<io::Error> for VMCError {
	fn from(value: io::Error) -> Self {
		Self::Io(value)
	}
}
impl From<osc::OSCError> for VMCError {
	fn from(value: osc::OSCError) -> Self {
		Self::Osc(value)
	}
}

impl Error for VMCError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			VMCError::Io(ref err) => Some(err),
			VMCError::Osc(ref err) => err.source(),
			_ => None
		}
	}
}

pub type VMCResult<T> = Result<T, VMCError>;
