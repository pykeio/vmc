use nom::{
	bytes::complete::{take, take_till},
	combinator::{map, map_parser, map_res},
	multi::many0,
	number::complete::{be_f32, be_f64, be_i32, be_i64, be_u32},
	sequence::{terminated, tuple},
	Err, IResult, Offset
};

use super::{
	error::{OSCError, OSCResult},
	OSCArray, OSCBundle, OSCColor, OSCMessage, OSCMidiMessage, OSCPacket, OSCTime, OSCType
};

/// Common MTU size for ethernet
pub const MTU: usize = 1536;

/// Takes a bytes slice representing a UDP packet and returns the OSC packet as well as a slice of
/// any bytes remaining after the OSC packet.
pub fn decode_udp(msg: &[u8]) -> OSCResult<(&[u8], OSCPacket)> {
	match decode_packet(msg, msg) {
		Ok((remainder, osc_packet)) => Ok((remainder, osc_packet)),
		Err(e) => match e {
			Err::Incomplete(_) => Err(OSCError::BadPacket("Incomplete data")),
			Err::Error(e) | Err::Failure(e) => Err(e)
		}
	}
}

/// Takes a bytes slice from a TCP stream (or any stream-based protocol) and returns the first OSC
/// packet as well as a slice of the bytes remaining after the packet.
///
/// # Difference to `decode_udp`
///
/// For stream-based protocols, such as TCP, the [OSC specification][^1] requires the size of
/// the first packet to be send as a 32-bit integer before the actual packet data.
///
/// [^1]: _In a stream-based protocol such as TCP, the stream should begin with an int32 giving the size of the first packet, followed by the contents of the first packet, followed by the size of the second packet, etc._
///
/// [OSC specification]: https://cnmat.org/OpenSoundControl/OSC-spec.html
pub fn decode_tcp(msg: &[u8]) -> OSCResult<(&[u8], Option<OSCPacket>)> {
	let (input, osc_packet_length) = match be_u32(msg) {
		Ok((i, o)) => (i, o),
		Err(e) => match e {
			Err::Incomplete(_) => return Err(OSCError::BadPacket("Incomplete data")),
			Err::Error(e) | Err::Failure(e) => return Err(e)
		}
	};

	if osc_packet_length as usize > msg.len() {
		return Ok((msg, None));
	}

	match decode_packet(input, msg).map(|(remainder, osc_packet)| (remainder, Some(osc_packet))) {
		Ok((remainder, osc_packet)) => Ok((remainder, osc_packet)),
		Err(e) => match e {
			Err::Incomplete(_) => Err(OSCError::BadPacket("Incomplete data")),
			Err::Error(e) | Err::Failure(e) => Err(e)
		}
	}
}

/// Takes a bytes slice from a TCP stream (or any stream-based protocol) and returns a vec of all
/// OSC packets in the slice as well as a slice of the bytes remaining after the last packet.
pub fn decode_tcp_vec(msg: &[u8]) -> OSCResult<(&[u8], Vec<OSCPacket>)> {
	let mut input = msg;
	let mut osc_packets = vec![];

	while let (remainder, Some(osc_packet)) = decode_tcp(input)? {
		input = remainder;
		osc_packets.push(osc_packet);

		if remainder.is_empty() {
			break;
		}
	}

	Ok((input, osc_packets))
}

fn decode_packet<'a>(input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], OSCPacket, OSCError> {
	if input.is_empty() {
		return Err(nom::Err::Error(OSCError::BadPacket("Empty packet.")));
	}

	let (input, addr) = read_osc_string(input, original_input)?;

	match addr.chars().next() {
		Some('/') => decode_message(addr, input, original_input),
		Some('#') if &addr == "#bundle" => decode_bundle(input, original_input),
		_ => Err(nom::Err::Error(OSCError::BadPacket("Invalid message address or bundle tag")))
	}
}

fn decode_message<'a>(addr: String, input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], OSCPacket, OSCError> {
	let (input, type_tags) = read_osc_string(input, original_input)?;

	if type_tags.len() > 1 {
		let (input, args) = read_osc_args(input, original_input, type_tags)?;
		Ok((input, OSCPacket::Message(OSCMessage { addr, args })))
	} else {
		Ok((input, OSCPacket::Message(OSCMessage { addr, args: vec![] })))
	}
}

fn decode_bundle<'a>(input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], OSCPacket, OSCError> {
	let (input, (timetag, content)) = tuple((read_time_tag, many0(|input| read_bundle_element(input, original_input))))(input)?;

	Ok((input, OSCPacket::Bundle(OSCBundle { timetag, content })))
}

fn read_bundle_element<'a>(input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], OSCPacket, OSCError> {
	let (input, elem_size) = be_u32(input)?;

	map_parser(
		move |input| take(elem_size)(input).map_err(|_: nom::Err<OSCError>| nom::Err::Error(OSCError::BadBundle("Bundle shorter than expected!".to_string()))),
		|input| decode_packet(input, original_input)
	)(input)
}

fn read_osc_string<'a>(input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], String, OSCError> {
	map_res(terminated(take_till(|c| c == 0u8), pad_to_32_bit_boundary(original_input)), |str_buf: &'a [u8]| {
		String::from_utf8(str_buf.into())
			.map_err(OSCError::StringError)
			.map(|s| s.trim_matches(0u8 as char).to_string())
	})(input)
}

fn read_osc_args<'a>(mut input: &'a [u8], original_input: &'a [u8], raw_type_tags: String) -> IResult<&'a [u8], Vec<OSCType>, OSCError> {
	let type_tags: Vec<char> = raw_type_tags.chars().skip(1).collect();

	let mut args: Vec<OSCType> = Vec::with_capacity(type_tags.len());
	let mut stack: Vec<Vec<OSCType>> = Vec::new();
	for tag in type_tags {
		if tag == '[' {
			// array start: save current frame and start a new frame
			// for the array's content
			stack.push(args);
			args = Vec::new();
		} else if tag == ']' {
			// found the end of the current array:
			// create array object from current frame and step one level up
			let array = OSCType::Array(OSCArray { content: args });
			match stack.pop() {
				Some(stashed) => args = stashed,
				None => return Err(nom::Err::Error(OSCError::BadMessage("Encountered ] outside array")))
			}
			args.push(array);
		} else {
			let input_and_arg = read_osc_arg(input, original_input, tag)?;
			input = input_and_arg.0;
			args.push(input_and_arg.1);
		}
	}
	Ok((input, args))
}

fn read_osc_arg<'a>(input: &'a [u8], original_input: &'a [u8], tag: char) -> IResult<&'a [u8], OSCType, OSCError> {
	match tag {
		'f' => map(be_f32, OSCType::Float)(input),
		'd' => map(be_f64, OSCType::Double)(input),
		'i' => map(be_i32, OSCType::Int)(input),
		'h' => map(be_i64, OSCType::Long)(input),
		's' => read_osc_string(input, original_input).map(|(remainder, string)| (remainder, OSCType::String(string))),
		't' => read_time_tag(input).map(|(remainder, time)| (remainder, OSCType::Time(time))),
		'b' => read_blob(input, original_input),
		'r' => read_osc_color(input),
		'T' => Ok((input, true.into())),
		'F' => Ok((input, false.into())),
		'N' => Ok((input, OSCType::Nil)),
		'I' => Ok((input, OSCType::Inf)),
		'c' => read_char(input),
		'm' => read_midi_message(input),
		_ => Err(nom::Err::Error(OSCError::BadArg(format!("Type tag \"{}\" is not implemented!", tag))))
	}
}

fn read_char(input: &[u8]) -> IResult<&[u8], OSCType, OSCError> {
	map_res(be_u32, |b| {
		let opt_char = char::from_u32(b);
		match opt_char {
			Some(c) => Ok(OSCType::Char(c)),
			None => Err(OSCError::BadArg("Argument is not a char!".to_string()))
		}
	})(input)
}

fn read_blob<'a>(input: &'a [u8], original_input: &'a [u8]) -> IResult<&'a [u8], OSCType, OSCError> {
	let (input, size) = be_u32(input)?;

	map(terminated(take(size), pad_to_32_bit_boundary(original_input)), |blob| OSCType::Blob(blob.into()))(input)
}

fn read_time_tag(input: &[u8]) -> IResult<&[u8], OSCTime, OSCError> {
	map(tuple((be_u32, be_u32)), |(seconds, fractional)| OSCTime { seconds, fractional })(input)
}

fn read_midi_message(input: &[u8]) -> IResult<&[u8], OSCType, OSCError> {
	map(take(4usize), |buf: &[u8]| {
		OSCType::Midi(OSCMidiMessage {
			port: buf[0],
			status: buf[1],
			data1: buf[2],
			data2: buf[3]
		})
	})(input)
}

fn read_osc_color(input: &[u8]) -> IResult<&[u8], OSCType, OSCError> {
	map(take(4usize), |buf: &[u8]| {
		OSCType::Color(OSCColor {
			red: buf[0],
			green: buf[1],
			blue: buf[2],
			alpha: buf[3]
		})
	})(input)
}

fn pad_to_32_bit_boundary<'a>(original_input: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (), OSCError> {
	move |input| {
		let offset = 4 - original_input.offset(input) % 4;
		let (input, _) = take(offset)(input)?;
		Ok((input, ()))
	}
}
