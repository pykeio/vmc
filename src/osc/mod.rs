use std::{
	convert::{TryFrom, TryInto},
	error::Error,
	fmt::{self, Display},
	time::{Duration, SystemTime, UNIX_EPOCH}
};

pub mod decoder;
pub mod encoder;
pub mod error;

pub use self::decoder::{decode_tcp, decode_tcp_vec, decode_udp, MTU};
pub use self::encoder::{encode, encode_into, encode_string, encode_string_into};
pub use self::error::{OSCError, OSCResult};

/// A time tag in OSC message consists of two 32-bit integers where the first one denotes the number of seconds since
/// 1900-01-01 and the second the fractions of a second. For details on its semantics see <http://opensoundcontrol.org/node/3/#timetags>
///
/// # Examples
///
/// ```
/// use std::{convert::TryFrom, time::UNIX_EPOCH};
///
/// use vmc::osc::OSCTime;
///
/// assert_eq!(OSCTime::try_from(UNIX_EPOCH).unwrap(), OSCTime::from((2_208_988_800, 0)));
/// ```
///
/// # Conversions between `(u32, u32)`
///
/// Prior to version `0.5.0` of this crate, `OSCTime` was defined as a type alias to `(u32, u32)`.
/// If you are upgrading from one of these older versions, you can use [`.into()`](Into::into) to
/// convert between `(u32, u32)` and `OSCTime` in either direction.
///
/// # Conversions between [`std::time::SystemTime`]
///
/// The traits in `std::convert` are implemented for converting between
/// [`SystemTime`](std::time::SystemTime) and `OSCTime` in both directions. An `OSCTime` can be
/// converted into a `SystemTime` using [`From`](std::convert::From)/[`Into`](std::convert::Into).
/// A `SystemTime` can be converted into an `OSCTime` using
/// [`TryFrom`](std::convert::TryFrom)/[`TryInto`](std::convert::TryInto). The fallible variants of
/// the conversion traits are used this case because not every `SystemTime` can be represented as
/// an `OSCTime`.
///
/// **These conversions are lossy**, but are tested to have a deviation within
/// 5 nanoseconds when converted back and forth in either direction.
///
/// Although any time since the OSC epoch (`1900-01-01 00:00:00 UTC`) can be represented using the
/// OSC timestamp format, this crate only allows conversions between times greater than or equal to
/// the [`UNIX_EPOCH`](std::time::UNIX_EPOCH). This allows the math used in the conversions to work
/// on 32-bit systems which cannot represent times that far back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OSCTime {
	pub seconds: u32,
	pub fractional: u32
}

impl OSCTime {
	const UNIX_OFFSET: u64 = 2_208_988_800; // From RFC 5905
	const TWO_POW_32: f64 = (u32::MAX as f64) + 1.0; // Number of bits in a `u32`
	const ONE_OVER_TWO_POW_32: f64 = 1.0 / OSCTime::TWO_POW_32;
	const NANOS_PER_SECOND: f64 = 1.0e9;
	const SECONDS_PER_NANO: f64 = 1.0 / OSCTime::NANOS_PER_SECOND;
}

impl TryFrom<SystemTime> for OSCTime {
	type Error = OSCTimeError;

	fn try_from(time: SystemTime) -> core::result::Result<OSCTime, OSCTimeError> {
		let duration_since_epoch =
			time.duration_since(UNIX_EPOCH).map_err(|_| OSCTimeError(OSCTimeErrorKind::BeforeEpoch))? + Duration::new(OSCTime::UNIX_OFFSET, 0);
		let seconds = u32::try_from(duration_since_epoch.as_secs()).map_err(|_| OSCTimeError(OSCTimeErrorKind::Overflow))?;
		let nanos = duration_since_epoch.subsec_nanos() as f64;
		let fractional = (nanos * OSCTime::SECONDS_PER_NANO * OSCTime::TWO_POW_32).round() as u32;
		Ok(OSCTime { seconds, fractional })
	}
}

impl From<OSCTime> for SystemTime {
	fn from(time: OSCTime) -> SystemTime {
		let nanos = (time.fractional as f64) * OSCTime::ONE_OVER_TWO_POW_32 * OSCTime::NANOS_PER_SECOND;
		let duration_since_osc_epoch = Duration::new(time.seconds as u64, nanos.round() as u32);
		let duration_since_unix_epoch = duration_since_osc_epoch - Duration::new(OSCTime::UNIX_OFFSET, 0);
		UNIX_EPOCH + duration_since_unix_epoch
	}
}

impl From<(u32, u32)> for OSCTime {
	fn from(time: (u32, u32)) -> OSCTime {
		let (seconds, fractional) = time;
		OSCTime { seconds, fractional }
	}
}

impl From<OSCTime> for (u32, u32) {
	fn from(time: OSCTime) -> (u32, u32) {
		(time.seconds, time.fractional)
	}
}

/// An error returned by conversions involving [`OSCTime`].
#[derive(Debug)]
pub struct OSCTimeError(OSCTimeErrorKind);

#[derive(Debug)]
enum OSCTimeErrorKind {
	BeforeEpoch,
	Overflow
}

impl Display for OSCTimeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			OSCTimeErrorKind::BeforeEpoch => {
				write!(f, "time is before the unix epoch and cannot be stored")
			}
			OSCTimeErrorKind::Overflow => {
				write!(f, "time overflows what OSC time can store")
			}
		}
	}
}

impl Error for OSCTimeError {}

/// see OSC Type Tag String: [OSC Spec. 1.0](http://opensoundcontrol.org/spec-1_0)
/// padding: zero bytes (n*4)
#[derive(Clone, Debug, PartialEq)]
pub enum OSCType {
	Int(i32),
	Float(f32),
	String(String),
	Blob(Vec<u8>),
	// use struct for time tag to avoid destructuring
	Time(OSCTime),
	Long(i64),
	Double(f64),
	Char(char),
	Color(OSCColor),
	Midi(OSCMidiMessage),
	Bool(bool),
	Array(OSCArray),
	Nil,
	Inf
}
macro_rules! value_impl {
    ($(($name:ident, $variant:ident, $ty:ty)),*) => {
        $(
        impl OSCType {
            #[allow(dead_code)]
            pub fn $name(self) -> Option<$ty> {
                match self {
                    OSCType::$variant(v) => Some(v),
                    _ => None
                }
            }
        }
        impl From<$ty> for OSCType {
            fn from(v: $ty) -> Self {
                OSCType::$variant(v)
            }
        }
        )*
    }
}
value_impl! {
	(int, Int, i32),
	(float, Float, f32),
	(string, String, String),
	(blob, Blob, Vec<u8>),
	(array, Array, OSCArray),
	(long, Long, i64),
	(double, Double, f64),
	(char, Char, char),
	(color, Color, OSCColor),
	(midi, Midi, OSCMidiMessage),
	(bool, Bool, bool)
}
impl From<(u32, u32)> for OSCType {
	fn from(time: (u32, u32)) -> Self {
		OSCType::Time(time.into())
	}
}

impl TryFrom<SystemTime> for OSCType {
	type Error = OSCTimeError;

	fn try_from(time: SystemTime) -> std::result::Result<OSCType, OSCTimeError> {
		time.try_into().map(OSCType::Time)
	}
}

impl OSCType {
	pub fn time(self) -> Option<OSCTime> {
		match self {
			OSCType::Time(time) => Some(time),
			_ => None
		}
	}
}
impl<'a> From<&'a str> for OSCType {
	fn from(string: &'a str) -> Self {
		OSCType::String(string.to_string())
	}
}
/// Represents the parts of a Midi message. Mainly used for
/// tunneling midi over a network using the OSC protocol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OSCMidiMessage {
	pub port: u8,
	pub status: u8,
	pub data1: u8, // maybe use an enum for data?
	pub data2: u8
}

/// An *osc packet* can contain an *osc message* or a bundle of nested messages
/// which is called *osc bundle*.
#[derive(Clone, Debug, PartialEq)]
pub enum OSCPacket {
	Message(OSCMessage),
	Bundle(OSCBundle)
}

impl OSCPacket {
	/// Return `Some(&message)` if the packet is 'OSCPacket::Message`.
	///
	/// Return None otherwise.
	pub fn message(&self) -> Option<&OSCMessage> {
		match self {
			OSCPacket::Message(message) => Some(message),
			_ => None
		}
	}

	/// Return `Some(message)` if the packet is 'OSCPacket::Message`.
	///
	/// Return None otherwise.
	pub fn into_message(self) -> Option<OSCMessage> {
		match self {
			OSCPacket::Message(message) => Some(message),
			_ => None
		}
	}
}

/// An OSC message consists of an address and
/// zero or more arguments. The address should
/// specify an element of your Instrument (or whatever
/// you want to control with OSC) and the arguments
/// are used to set properties of the element to the
/// respective values.
#[derive(Clone, Debug, PartialEq)]
pub struct OSCMessage {
	pub addr: String,
	pub args: Vec<OSCType>
}

impl OSCMessage {
	/// Create a new OSCMessage from an address and args.
	/// The args can either be specified as a `Vec<[OSCType]`, or as a tuple of regular Rust types
	/// that can be converted into [`OSCType`].
	pub fn new<T>(addr: impl ToString, args: T) -> Self
	where
		T: IntoOSCArgs
	{
		let args = args.into_osc_args();
		let addr = addr.to_string();
		OSCMessage { addr, args }
	}

	/// Returns `true` if the address starts with the given prefix.
	///
	/// Returns `false` otherwise.
	pub fn starts_with(&self, prefix: &str) -> bool {
		self.addr.starts_with(prefix)
	}

	/// Get a reference to the message in tuple form.
	///
	/// This is useful for pattern matching. Example:
	///
	/// ```no_run
	/// # use vmc::osc::{OSCMessage, OSCType};
	/// let message = OSCMessage::new("/foo", vec![OSCType::Float(1.0), OSCType::String("bar".into())]);
	///
	/// match message.as_tuple() {
	/// 	("foo", &[OSCType::Float(val), OSCType::String(ref text)]) => {
	/// 		eprintln!("Got foo message with args: {}, {}", val, text);
	/// 	}
	/// 	_ => {}
	/// }
	/// ```
	pub fn as_tuple(&self) -> (&str, &[OSCType]) {
		(self.addr.as_str(), &self.args[..])
	}
}

/// An OSC bundle contains zero or more OSC packets
/// and a time tag. The contained packets *should* be
/// applied at the given time tag.
#[derive(Clone, Debug, PartialEq)]
pub struct OSCBundle {
	pub timetag: OSCTime,
	pub content: Vec<OSCPacket>
}

/// An RGBA color.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OSCColor {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

/// An OSCArray.
#[derive(Clone, Debug, PartialEq)]
pub struct OSCArray {
	pub content: Vec<OSCType>
}

impl<T: Into<OSCType>> FromIterator<T> for OSCArray {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OSCArray {
		OSCArray {
			content: iter.into_iter().map(T::into).collect()
		}
	}
}

impl From<String> for OSCMessage {
	fn from(s: String) -> OSCMessage {
		OSCMessage { addr: s, args: vec![] }
	}
}

impl<'a> From<&'a str> for OSCMessage {
	fn from(s: &str) -> OSCMessage {
		OSCMessage { addr: s.to_string(), args: vec![] }
	}
}

/// Helper trait to convert types into `Vec<[OSCType]>`
pub trait IntoOSCArgs {
	/// Convert self to OSC args.
	fn into_osc_args(self) -> Vec<OSCType>;
}

impl<T> IntoOSCArgs for Vec<T>
where
	T: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		let args: Vec<OSCType> = self.into_iter().map(|a| a.into()).collect();
		args
	}
}

impl IntoOSCArgs for () {
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![]
	}
}

impl<T1> IntoOSCArgs for (T1,)
where
	T1: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into()]
	}
}

impl<T1, T2> IntoOSCArgs for (T1, T2)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into()]
	}
}

impl<T1, T2, T3> IntoOSCArgs for (T1, T2, T3)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into()]
	}
}

impl<T1, T2, T3, T4> IntoOSCArgs for (T1, T2, T3, T4)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>,
	T4: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
	}
}

impl<T1, T2, T3, T4, T5> IntoOSCArgs for (T1, T2, T3, T4, T5)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>,
	T4: Into<OSCType>,
	T5: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()]
	}
}

impl<T1, T2, T3, T4, T5, T6> IntoOSCArgs for (T1, T2, T3, T4, T5, T6)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>,
	T4: Into<OSCType>,
	T5: Into<OSCType>,
	T6: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()]
	}
}

impl<T1, T2, T3, T4, T5, T6, T7> IntoOSCArgs for (T1, T2, T3, T4, T5, T6, T7)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>,
	T4: Into<OSCType>,
	T5: Into<OSCType>,
	T6: Into<OSCType>,
	T7: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into()]
	}
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> IntoOSCArgs for (T1, T2, T3, T4, T5, T6, T7, T8)
where
	T1: Into<OSCType>,
	T2: Into<OSCType>,
	T3: Into<OSCType>,
	T4: Into<OSCType>,
	T5: Into<OSCType>,
	T6: Into<OSCType>,
	T7: Into<OSCType>,
	T8: Into<OSCType>
{
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into()]
	}
}

impl IntoOSCArgs for OSCType {
	fn into_osc_args(self) -> Vec<OSCType> {
		vec![self]
	}
}

/// Helper trait to convert [`OSCMessage`] and [`OSCBundle`] into [`OSCPacket`].
pub trait IntoOSCPacket {
	/// Convert into [`OSCPacket`].
	fn into_osc_packet(self) -> OSCPacket;
}

impl IntoOSCPacket for OSCMessage {
	fn into_osc_packet(self) -> OSCPacket {
		OSCPacket::Message(self)
	}
}

impl IntoOSCPacket for OSCBundle {
	fn into_osc_packet(self) -> OSCPacket {
		OSCPacket::Bundle(self)
	}
}

impl IntoOSCPacket for OSCPacket {
	fn into_osc_packet(self) -> OSCPacket {
		self
	}
}

impl<T> IntoOSCPacket for T
where
	T: IntoOSCMessage
{
	fn into_osc_packet(self) -> OSCPacket {
		OSCPacket::Message(self.into_osc_message())
	}
}

/// Helper trait to convert a `(impl ToString, impl IntoOSCArgs)` tuple into [`OSCMessage`].
pub trait IntoOSCMessage {
	/// Convert to [`OSCMessage`].
	fn into_osc_message(self) -> OSCMessage;
}

impl<S, A> IntoOSCMessage for (S, A)
where
	S: ToString,
	A: IntoOSCArgs
{
	fn into_osc_message(self) -> OSCMessage {
		OSCMessage::new(self.0, self.1)
	}
}
