use super::{error::OSCResult, OSCBundle, OSCMessage, OSCPacket, OSCTime, OSCType};

/// Takes a reference to an OSC packet and returns
/// a byte vector on success. If the packet was invalid
/// an `OSCError` is returned.
///
/// # Example
///
/// ```
/// use vmc::osc::{encoder, OSCMessage, OSCPacket, OSCType};
///
/// let packet = OSCPacket::Message(OSCMessage {
/// 	addr: "/greet/me".to_string(),
/// 	args: vec![OSCType::String("hi!".to_string())]
/// });
/// assert!(encoder::encode(&packet).is_ok())
/// ```
pub fn encode(packet: &OSCPacket) -> OSCResult<Vec<u8>> {
	let mut bytes = Vec::new();

	// NOTE: The Output implementation for Vec<u8> can't actually produce an error!
	encode_into(packet, &mut bytes).expect("Failed to write encoded packet into Vec");

	Ok(bytes)
}

/// Takes a reference to an OSC packet and writes the
/// encoded bytes to the given output. On success, the
/// number of bytes written will be returned. If an error
/// occurs during encoding, encoding will stop and the
/// error will be returned. Note that in that case, the
/// output may have been partially written!
///
/// NOTE: The OSC encoder will write output in small pieces
/// (as small as a single byte), so the output should be
/// buffered if write calls have a large overhead (e.g.
/// writing to a file).
///
/// # Example
///
/// ```
/// use vmc::osc::{encoder, OSCMessage, OSCPacket, OSCType};
///
/// let mut bytes = Vec::new();
/// let packet = OSCPacket::Message(OSCMessage {
/// 	addr: "/greet/me".to_string(),
/// 	args: vec![OSCType::String("hi!".to_string())]
/// });
/// assert!(encoder::encode_into(&packet, &mut bytes).is_ok())
/// ```
pub fn encode_into<O: Output>(packet: &OSCPacket, out: &mut O) -> Result<usize, O::Err> {
	match *packet {
		OSCPacket::Message(ref msg) => encode_message(msg, out),
		OSCPacket::Bundle(ref bundle) => encode_bundle(bundle, out)
	}
}

fn encode_message<O: Output>(msg: &OSCMessage, out: &mut O) -> Result<usize, O::Err> {
	let mut written = encode_string_into(&msg.addr, out)?;

	written += out.write(b",")?;
	for arg in &msg.args {
		written += encode_arg_type(arg, out)?;
	}

	let padding = pad(written as u64 + 1) as usize - written;
	written += out.write(&[0u8; 4][..padding])?;

	for arg in &msg.args {
		written += encode_arg_data(arg, out)?;
	}

	Ok(written)
}

fn encode_bundle<O: Output>(bundle: &OSCBundle, out: &mut O) -> Result<usize, O::Err> {
	let mut written = encode_string_into("#bundle", out)?;
	written += encode_time_tag_into(&bundle.timetag, out)?;

	for packet in &bundle.content {
		match *packet {
			OSCPacket::Message(ref m) => {
				let length_mark = out.mark(4)?;

				let length = encode_message(m, out)?;
				out.place(length_mark, &(length as u32).to_be_bytes())?;

				written += 4 + length;
			}
			OSCPacket::Bundle(ref b) => {
				let length_mark = out.mark(4)?;

				let length = encode_bundle(b, out)?;
				out.place(length_mark, &(length as u32).to_be_bytes())?;

				written += 4 + length;
			}
		}
	}

	Ok(written)
}

fn encode_arg_data<O: Output>(arg: &OSCType, out: &mut O) -> Result<usize, O::Err> {
	match *arg {
		OSCType::Int(x) => out.write(&x.to_be_bytes()),
		OSCType::Long(x) => out.write(&x.to_be_bytes()),
		OSCType::Float(x) => out.write(&x.to_be_bytes()),
		OSCType::Double(x) => out.write(&x.to_be_bytes()),
		OSCType::Char(x) => out.write(&(x as u32).to_be_bytes()),
		OSCType::String(ref x) => encode_string_into(x, out),
		OSCType::Blob(ref x) => {
			let padded_blob_length = pad(x.len() as u64) as usize;
			let padding = padded_blob_length - x.len();

			out.write(&(x.len() as u32).to_be_bytes())?;
			out.write(x)?;

			if padding > 0 {
				out.write(&[0u8; 3][..padding])?;
			}

			Ok(4 + padded_blob_length)
		}
		OSCType::Time(ref time) => encode_time_tag_into(time, out),
		OSCType::Midi(ref x) => out.write(&[x.port, x.status, x.data1, x.data2]),
		OSCType::Color(ref x) => out.write(&[x.red, x.green, x.blue, x.alpha]),
		OSCType::Bool(_) => Ok(0),
		OSCType::Nil => Ok(0),
		OSCType::Inf => Ok(0),
		OSCType::Array(ref x) => {
			let mut written = 0;
			for v in &x.content {
				written += encode_arg_data(v, out)?;
			}
			Ok(written)
		}
	}
}

fn encode_arg_type<O: Output>(arg: &OSCType, out: &mut O) -> Result<usize, O::Err> {
	match *arg {
		OSCType::Int(_) => out.write(b"i"),
		OSCType::Long(_) => out.write(b"h"),
		OSCType::Float(_) => out.write(b"f"),
		OSCType::Double(_) => out.write(b"d"),
		OSCType::Char(_) => out.write(b"c"),
		OSCType::String(_) => out.write(b"s"),
		OSCType::Blob(_) => out.write(b"b"),
		OSCType::Time(_) => out.write(b"t"),
		OSCType::Midi(_) => out.write(b"m"),
		OSCType::Color(_) => out.write(b"r"),
		OSCType::Bool(x) => out.write(if x { b"T" } else { b"F" }),
		OSCType::Nil => out.write(b"N"),
		OSCType::Inf => out.write(b"I"),
		OSCType::Array(ref x) => {
			let mut written = out.write(b"[")?;

			for v in &x.content {
				written += encode_arg_type(v, out)?;
			}

			written += out.write(b"]")?;
			Ok(written)
		}
	}
}

/// Null terminates the byte representation of string `s` and
/// adds null bytes until the length of the result is a
/// multiple of 4.
pub fn encode_string<S: Into<String>>(s: S) -> Vec<u8> {
	let mut bytes: Vec<u8> = s.into().into_bytes();

	let new_len = pad(bytes.len() as u64 + 1) as usize;
	bytes.resize(new_len, 0u8);

	bytes
}

/// Writes the given string `s` to the given Output, adding
/// 1-4 null bytes such that the length of the result is a
/// multiple of 4.
pub fn encode_string_into<S: AsRef<str>, O: Output>(s: S, out: &mut O) -> Result<usize, O::Err> {
	let s = s.as_ref();

	let padded_len = pad(s.len() as u64 + 1) as usize;
	let padding = padded_len - s.len();
	out.write(s.as_bytes())?;
	out.write(&[0u8; 4][..padding])?;
	Ok(s.len() + padding)
}

/// Returns the position padded to 4 bytes.
///
/// # Example
///
/// ```
/// use vmc::osc::encoder;
///
/// let pos: u64 = 10;
/// assert_eq!(12u64, encoder::pad(pos))
/// ```
pub fn pad(pos: u64) -> u64 {
	match pos % 4 {
		0 => pos,
		d => pos + (4 - d)
	}
}

fn encode_time_tag_into<O: Output>(time: &OSCTime, out: &mut O) -> Result<usize, O::Err> {
	out.write(&time.seconds.to_be_bytes())?;
	out.write(&time.fractional.to_be_bytes())?;
	Ok(8)
}

/// A trait for values that can receive encoded OSC output
/// via `encode_into`. This allows more flexibility in how
/// the output is handled, including reusing part of an
/// existing buffer or writing directly to an external sink
/// (e.g. a file).
///
/// Implementations are currently provided for this trait for:
/// - `Vec<u8>`: Data will be appended to the end of the Vec.
/// - `WriteOutput<W>` (with feature `std`): A wrapper that allows data to be written to any type that implements
///   `std::io::Seek + std::io::Write`.
pub trait Output {
	/// The error type which is returned from Output functions.
	type Err;

	/// The type which should be used to indicate the location of a mark.
	type Mark;

	/// Writes a block of data to the output.
	///
	/// Note that, unlike `std::io::Write::write`, this
	/// function is expected to write all of the given data prior to returning.
	fn write(&mut self, data: &[u8]) -> Result<usize, Self::Err>;

	/// Marks the location of a fixed-length value and returns a `Self::Mark` which may be used to
	/// fill in its data later with `place`.
	fn mark(&mut self, size: usize) -> Result<Self::Mark, Self::Err>;

	/// Consumes a previously-generated Mark and fills it in with data.
	///
	/// This may result in a panic or in invalid data being written if `mark` came from a different
	/// `Output`, or if the length of `data` does not match the size passed to `mark`.
	fn place(&mut self, mark: Self::Mark, data: &[u8]) -> Result<(), Self::Err>;
}

impl Output for Vec<u8> {
	type Err = core::convert::Infallible;
	type Mark = (usize, usize);

	#[inline]
	fn mark(&mut self, size: usize) -> Result<Self::Mark, Self::Err> {
		let start = self.len();
		let end = start + size;

		self.resize(end, 0);
		Ok((start, end))
	}

	#[inline]
	fn place(&mut self, (start, end): Self::Mark, data: &[u8]) -> Result<(), Self::Err> {
		self[start..end].copy_from_slice(data);
		Ok(())
	}

	#[inline]
	fn write(&mut self, data: &[u8]) -> Result<usize, Self::Err> {
		self.extend(data);
		Ok(data.len())
	}
}

/// A new type which can be used to wrap any type which
/// implements `std::io::Seek` and `std::io::Write` to allow
/// it to be used as an `Output`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WriteOutput<W>(pub W);

impl<W: std::io::Seek + std::io::Write> Output for WriteOutput<W> {
	type Err = std::io::Error;
	type Mark = u64;

	fn mark(&mut self, size: usize) -> Result<Self::Mark, Self::Err> {
		let pos = self.0.stream_position()?;

		let mut left = size;
		while left > 0 {
			let num = left.min(8);
			self.0.write_all(&[0; 8][..num])?;
			left -= num;
		}

		Ok(pos)
	}

	fn place(&mut self, pos: Self::Mark, data: &[u8]) -> Result<(), Self::Err> {
		let old_pos = self.0.stream_position()?;

		self.0.seek(std::io::SeekFrom::Start(pos))?;
		self.0.write_all(data)?;
		self.0.seek(std::io::SeekFrom::Start(old_pos))?;

		Ok(())
	}

	#[inline]
	fn write(&mut self, data: &[u8]) -> Result<usize, Self::Err> {
		std::io::Write::write_all(&mut self.0, data).map(|_| data.len())
	}
}
