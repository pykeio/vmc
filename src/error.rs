use std::{
	fmt,
	io::{self}
};

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Osc(rosc::OscError),
	UnimplementedMessage(String, Vec<rosc::OscType>),
	UnknownBone(String),
	UnknownBlendShape(String),
	UnknownModelState(i32),
	UnknownCalibrationState(i32),
	UnknownCalibrationMode(i32),
	UnknownTrackingState(i32)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Io(err) => write!(f, "socket error: {err}"),
			Error::Osc(err) => write!(f, "protocol error: {err}"),
			Error::UnimplementedMessage(addr, args) => write!(f, "handling '{addr}' not implemented (args: {args:?})"),
			Error::UnknownBone(bone) => write!(f, "unknown bone: {bone}"),
			Error::UnknownBlendShape(blend_shape) => write!(f, "unknown blend shape: {blend_shape}"),
			Error::UnknownModelState(state) => write!(f, "unknown model state: {state}"),
			Error::UnknownCalibrationState(state) => write!(f, "unknown calibration state: {state}"),
			Error::UnknownCalibrationMode(mode) => write!(f, "unknown calibration mode: {mode}"),
			Error::UnknownTrackingState(state) => write!(f, "unknown tracking state: {state}")
		}
	}
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Self::Io(value)
	}
}
impl From<rosc::OscError> for Error {
	fn from(value: rosc::OscError) -> Self {
		Self::Osc(value)
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Io(ref err) => Some(err),
			Error::Osc(ref err) => err.source(),
			_ => None
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
