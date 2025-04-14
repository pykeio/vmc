//! Submodule for Virtual Motion Capture-specific messages.

use std::{str::FromStr, sync::OnceLock, time::Instant};

use rosc::{OscMessage, OscPacket, OscType};

use crate::{
	Error, IntoOSCMessage, Result,
	definitions::{CalibrationMode, CalibrationState, DeviceType, ModelState, Quat, StandardVRM0Bone, TrackingState, Vec3}
};

/// Root Transform message (`/VMC/Ext/Root/Pos`)
///
/// Changes the model root absolute position, rotation, and optionally, scale & offset.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootTransform {
	pub position: Vec3,
	pub rotation: Quat,
	pub scale: Option<Vec3>,
	pub offset: Option<Vec3>
}

impl RootTransform {
	/// Creates a new root transform message.
	pub fn new(position: impl Into<Vec3>, rotation: impl Into<Quat>) -> Self {
		Self {
			position: position.into(),
			rotation: rotation.into(),
			scale: None,
			offset: None
		}
	}

	/// Creates a new root transform message with additional scale & offset parameters, which can be used to adjust the
	/// size and position of the virtual avatar to match the physical body.
	pub fn new_mr(position: impl Into<Vec3>, rotation: impl Into<Quat>, scale: impl Into<Vec3>, offset: impl Into<Vec3>) -> Self {
		Self {
			position: position.into(),
			rotation: rotation.into(),
			scale: Some(scale.into()),
			offset: Some(offset.into())
		}
	}
}

impl IntoOSCMessage for RootTransform {
	fn into_osc_message(self) -> OscMessage {
		let mut args: Vec<OscType> = vec![
			"root".into(),
			self.position.x.into(),
			self.position.y.into(),
			self.position.z.into(),
			self.rotation.x.into(),
			self.rotation.y.into(),
			self.rotation.z.into(),
			self.rotation.w.into(),
		];
		if let (Some(scale), Some(offset)) = (self.scale.as_ref(), self.offset.as_ref()) {
			args.extend([scale.x.into(), scale.y.into(), scale.z.into()]);
			args.extend([offset.x.into(), offset.y.into(), offset.z.into()]);
		}
		OscMessage {
			addr: "/VMC/Ext/Root/Pos".to_string(),
			args
		}
	}
}

/// Bone Transform message (`/VMC/Ext/Bone/Pos`)
///
/// Used to adjust the position and rotation of humanoid bones.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoneTransform {
	pub bone: String,
	pub position: Vec3,
	pub rotation: Quat
}

impl BoneTransform {
	/// Creates a new bone transform message.
	///
	/// `bone` is the name of the bone; see [`StandardVRM0Bone`] for standard VRM 0.x bone names.
	pub fn new(bone: impl ToString, position: impl Into<Vec3>, rotation: impl Into<Quat>) -> Self {
		Self {
			bone: bone.to_string(),
			position: position.into(),
			rotation: rotation.into()
		}
	}
}

impl IntoOSCMessage for BoneTransform {
	fn into_osc_message(self) -> OscMessage {
		OscMessage {
			addr: "/VMC/Ext/Bone/Pos".to_string(),
			args: vec![
				OscType::String(self.bone),
				OscType::Float(self.position.x),
				OscType::Float(self.position.y),
				OscType::Float(self.position.z),
				OscType::Float(self.rotation.x),
				OscType::Float(self.rotation.y),
				OscType::Float(self.rotation.z),
				OscType::Float(self.rotation.w),
			]
		}
	}
}

/// Device Transform message (`/VMC/Ext/{Hmd,Con,Tra}/Pos`)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceTransform {
	pub device: DeviceType,
	pub joint: String,
	pub position: Vec3,
	pub rotation: Quat,
	pub local: bool
}

impl DeviceTransform {
	/// Creates a new device transform message.
	///
	/// - `joint` is the OpenVR serial no.
	/// - `local` determines whether the position is in raw device scale (`true`) or avatar scale (`false`).
	pub fn new(device: DeviceType, joint: impl ToString, position: impl Into<Vec3>, rotation: impl Into<Quat>, local: bool) -> Self {
		Self {
			device,
			joint: joint.to_string(),
			position: position.into(),
			rotation: rotation.into(),
			local
		}
	}
}

impl IntoOSCMessage for DeviceTransform {
	fn into_osc_message(self) -> OscMessage {
		OscMessage {
			addr: format!("/VMC/Ext/{}/Pos{}", self.device.as_ref(), if self.local { "/Local" } else { "" }),
			args: vec![
				OscType::String(self.joint),
				OscType::Float(self.position.x),
				OscType::Float(self.position.y),
				OscType::Float(self.position.z),
				OscType::Float(self.rotation.x),
				OscType::Float(self.rotation.y),
				OscType::Float(self.rotation.z),
				OscType::Float(self.rotation.w),
			]
		}
	}
}

/// Blend Shape message (`/VMC/Ext/Blend/Val`)
///
/// Note that blendshapes will not update until you send [`ApplyBlendShapes`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlendShape {
	pub key: String,
	pub value: f32
}

impl BlendShape {
	/// Creates a new blendshape message.
	///
	/// See [`StandardVRMBlendShape`](crate::StandardVRMBlendShape) for standard blendshapes.
	pub fn new(key: impl ToString, value: f32) -> Self {
		Self { key: key.to_string(), value }
	}
}

impl IntoOSCMessage for BlendShape {
	fn into_osc_message(self) -> OscMessage {
		OscMessage {
			addr: "/VMC/Ext/Blend/Val".to_string(),
			args: vec![OscType::String(self.key), OscType::Float(self.value)]
		}
	}
}

/// Apply Blend Shape message (`/VMC/Ext/Blend/Apply`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyBlendShapes;

impl IntoOSCMessage for ApplyBlendShapes {
	fn into_osc_message(self) -> OscMessage {
		OscMessage {
			addr: "/VMC/Ext/Blend/Apply".to_string(),
			args: Vec::new()
		}
	}
}

/// State/Availability message (`/VMC/Ext/OK`)
///
/// Used to send information like model, calibration, & tracking status.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State {
	pub model_state: ModelState,
	pub calibration_state: Option<(CalibrationMode, CalibrationState)>,
	pub tracking_state: Option<TrackingState>
}

impl State {
	/// Creates a new status message containing only the model loading state.
	pub fn new(model_state: ModelState) -> State {
		Self {
			model_state,
			calibration_state: None,
			tracking_state: None
		}
	}

	/// Creates a new status message containing the model loading state and calibration mode & status.
	pub fn new_calibration(model_state: ModelState, calibration_mode: CalibrationMode, calibration_state: CalibrationState) -> State {
		Self {
			model_state,
			calibration_state: Some((calibration_mode, calibration_state)),
			tracking_state: None
		}
	}

	/// Creates a new status message containing the model, calibration, & tracking status.
	pub fn new_tracking(
		model_state: ModelState,
		calibration_mode: CalibrationMode,
		calibration_state: CalibrationState,
		tracking_state: TrackingState
	) -> State {
		Self {
			model_state,
			calibration_state: Some((calibration_mode, calibration_state)),
			tracking_state: Some(tracking_state)
		}
	}
}

impl IntoOSCMessage for State {
	fn into_osc_message(self) -> OscMessage {
		let mut args: Vec<OscType> = vec![self.model_state.into()];
		if let Some((calibration_mode, calibration_state)) = self.calibration_state {
			args.extend([calibration_state.into(), calibration_mode.into()]);
			if let Some(tracking_state) = self.tracking_state {
				args.push(tracking_state.into());
			}
		}
		OscMessage {
			addr: "/VMC/Ext/OK".to_string(),
			args
		}
	}
}

/// Relative Time message (`/VMC/Ext/T`)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time(pub f32);

impl Time {
	pub fn new(timestamp: f32) -> Self {
		Self(timestamp)
	}

	/// Creates a new time message, automatically tracking relative time using a monotonic clock.
	pub fn elapsed() -> Self {
		static EPOCH: OnceLock<Instant> = OnceLock::new();
		Self(EPOCH.get_or_init(Instant::now).elapsed().as_secs_f32())
	}
}

impl IntoOSCMessage for Time {
	fn into_osc_message(self) -> OscMessage {
		OscMessage {
			addr: "/VMC/Ext/T".to_string(),
			args: vec![OscType::Float(self.0)]
		}
	}
}

/// Contains any possible message that can be sent over VMC protocol.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Message {
	RootTransform(RootTransform),
	DeviceTransform(DeviceTransform),
	BoneTransform(BoneTransform),
	BlendShape(BlendShape),
	ApplyBlendShapes,
	State(State),
	Time(Time)
}

impl IntoOSCMessage for Message {
	fn into_osc_message(self) -> OscMessage {
		match self {
			Self::RootTransform(p) => p.into_osc_message(),
			Self::DeviceTransform(p) => p.into_osc_message(),
			Self::BoneTransform(p) => p.into_osc_message(),
			Self::BlendShape(p) => p.into_osc_message(),
			Self::ApplyBlendShapes => ApplyBlendShapes.into_osc_message(),
			Self::State(p) => p.into_osc_message(),
			Self::Time(p) => p.into_osc_message()
		}
	}
}

impl From<RootTransform> for Message {
	fn from(value: RootTransform) -> Self {
		Self::RootTransform(value)
	}
}
impl From<DeviceTransform> for Message {
	fn from(value: DeviceTransform) -> Self {
		Self::DeviceTransform(value)
	}
}
impl From<BoneTransform> for Message {
	fn from(value: BoneTransform) -> Self {
		Self::BoneTransform(value)
	}
}
impl From<BlendShape> for Message {
	fn from(value: BlendShape) -> Self {
		Self::BlendShape(value)
	}
}
impl From<ApplyBlendShapes> for Message {
	fn from(_value: ApplyBlendShapes) -> Self {
		Self::ApplyBlendShapes
	}
}
impl From<State> for Message {
	fn from(value: State) -> Self {
		Self::State(value)
	}
}
impl From<Time> for Message {
	fn from(value: Time) -> Self {
		Self::Time(value)
	}
}

fn flatten_packet(packet: OscPacket) -> Vec<OscMessage> {
	match packet {
		OscPacket::Bundle(bundle) => bundle.content.into_iter().flat_map(flatten_packet).collect(),
		OscPacket::Message(message) => vec![message]
	}
}

/// Parses an [`OSCPacket`] into its contained VMC [`Message`]s. This will automatically flatten message bundles and
/// handle the parsing to different message types. Returns an error upon encountering an unimplemented packet.
pub fn parse(osc_packet: OscPacket) -> Result<Vec<Message>> {
	let messages = flatten_packet(osc_packet);
	messages
		.into_iter()
		.map(|msg| match (&*msg.addr, &*msg.args) {
			(
				"/VMC/Ext/Root/Pos",
				&[
					OscType::String(_),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w)
				]
			) => Ok(Message::RootTransform(RootTransform::new(Vec3::new(p_x, p_y, p_z), Quat::from_xyzw(r_x, r_y, r_z, r_w)))),
			(
				"/VMC/Ext/Root/Pos",
				&[
					OscType::String(_),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					OscType::Float(s_x),
					OscType::Float(s_y),
					OscType::Float(s_z),
					OscType::Float(o_x),
					OscType::Float(o_y),
					OscType::Float(o_z),
					..
				]
			) => Ok(Message::RootTransform(RootTransform::new_mr(
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				Vec3::new(s_x, s_y, s_z),
				Vec3::new(o_x, o_y, o_z)
			))),
			(
				"/VMC/Ext/Bone/Pos",
				&[
					OscType::String(ref bone),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w)
				]
			) => Ok(Message::BoneTransform(BoneTransform::new(
				StandardVRM0Bone::from_str(bone).map_err(|_| Error::UnknownBone(bone.to_string()))?,
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w)
			))),
			(
				"/VMC/Ext/Hmd/Pos",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::HMD,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				false
			))),
			(
				"/VMC/Ext/Hmd/Pos/Local",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::HMD,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				true
			))),
			(
				"/VMC/Ext/Con/Pos",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::Controller,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				false
			))),
			(
				"/VMC/Ext/Con/Pos/Local",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::Controller,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				true
			))),
			(
				"/VMC/Ext/Tra/Pos",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::Tracker,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				false
			))),
			(
				"/VMC/Ext/Tra/Pos/Local",
				&[
					OscType::String(ref joint),
					OscType::Float(p_x),
					OscType::Float(p_y),
					OscType::Float(p_z),
					OscType::Float(r_x),
					OscType::Float(r_y),
					OscType::Float(r_z),
					OscType::Float(r_w),
					..
				]
			) => Ok(Message::DeviceTransform(DeviceTransform::new(
				DeviceType::Tracker,
				joint.to_owned(),
				Vec3::new(p_x, p_y, p_z),
				Quat::from_xyzw(r_x, r_y, r_z, r_w),
				true
			))),
			("/VMC/Ext/Blend/Val", &[OscType::String(ref shape), OscType::Float(val), ..]) => Ok(Message::BlendShape(BlendShape::new(shape, val))),
			("/VMC/Ext/Blend/Apply", &[..]) => Ok(Message::ApplyBlendShapes),
			("/VMC/Ext/OK", &[OscType::Int(model_state)]) => Ok(Message::State(State::new(model_state.try_into().map_err(Error::UnknownModelState)?))),
			("/VMC/Ext/OK", &[OscType::Int(model_state), OscType::Int(calibration_state), OscType::Int(calibration_mode)]) => {
				Ok(Message::State(State::new_calibration(
					model_state.try_into().map_err(Error::UnknownModelState)?,
					calibration_mode.try_into().map_err(Error::UnknownCalibrationMode)?,
					calibration_state.try_into().map_err(Error::UnknownCalibrationState)?
				)))
			}
			(
				"/VMC/Ext/OK",
				&[
					OscType::Int(model_state),
					OscType::Int(calibration_state),
					OscType::Int(calibration_mode),
					OscType::Int(tracking_state),
					..
				]
			) => Ok(Message::State(State::new_tracking(
				model_state.try_into().map_err(Error::UnknownModelState)?,
				calibration_mode.try_into().map_err(Error::UnknownCalibrationMode)?,
				calibration_state.try_into().map_err(Error::UnknownCalibrationState)?,
				tracking_state.try_into().map_err(Error::UnknownTrackingState)?
			))),
			("/VMC/Ext/T", &[OscType::Float(time), ..]) => Ok(Message::Time(Time::new(time))),
			(addr, args) => Err(Error::UnimplementedMessage(addr.to_owned(), args.to_owned()))
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use approx::assert_relative_eq;

	use super::*;
	use crate::{IntoOSCPacket, StandardVRMBlendShape};

	#[test]
	fn test_parse_root_transform() -> Result<()> {
		let position = Vec3::new(0.5, 0.2, -0.4);
		let rotation = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
		let scale = Vec3::new(0.8, 1.0, 0.3);
		let offset = Vec3::new(-0.1, 0.12, -0.3);

		let packet = RootTransform::new(position, rotation).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::RootTransform(transform) => {
				assert_relative_eq!(transform.position, position);
				assert_relative_eq!(transform.rotation, rotation);
				assert!(transform.scale.is_none());
				assert!(transform.offset.is_none());
			}
			_ => panic!()
		}

		let packet = RootTransform::new_mr(position, rotation, scale, offset).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::RootTransform(transform) => {
				assert_relative_eq!(transform.position, position);
				assert_relative_eq!(transform.rotation, rotation);
				assert_relative_eq!(transform.scale.unwrap(), scale);
				assert_relative_eq!(transform.offset.unwrap(), offset);
			}
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_parse_bone_transform() -> Result<()> {
		let position = Vec3::new(0.5, 0.2, -0.4);
		let rotation = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);

		for bone in [
			StandardVRM0Bone::Chest,
			StandardVRM0Bone::RightEye,
			StandardVRM0Bone::LeftIndexDistal,
			StandardVRM0Bone::Spine,
			StandardVRM0Bone::RightHand
		] {
			let packet = BoneTransform::new(bone, position, rotation).into_osc_packet();
			let parsed_packet = &parse(packet)?[0];
			match parsed_packet {
				Message::BoneTransform(transform) => {
					assert_eq!(transform.bone, bone);
					assert_relative_eq!(transform.position, position);
					assert_relative_eq!(transform.rotation, rotation);
				}
				_ => panic!()
			}
		}

		Ok(())
	}

	#[test]
	fn test_parse_device_transform() -> Result<()> {
		let position = Vec3::new(0.5, 0.2, -0.4);
		let rotation = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);

		for device in [DeviceType::HMD, DeviceType::Controller, DeviceType::Tracker] {
			for joint in ["Head", "LeftHand"] {
				for locality in [true, false] {
					let packet = DeviceTransform::new(device, joint, position, rotation, locality).into_osc_packet();
					let parsed_packet = &parse(packet)?[0];
					match parsed_packet {
						Message::DeviceTransform(transform) => {
							assert_eq!(transform.device, device);
							assert_eq!(transform.joint, joint);
							assert_relative_eq!(transform.position, position);
							assert_relative_eq!(transform.rotation, rotation);
							assert_eq!(transform.local, locality);
						}
						_ => panic!()
					}
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_parse_blend_shape() -> Result<()> {
		for shape in [StandardVRMBlendShape::A, StandardVRMBlendShape::LookRight, StandardVRMBlendShape::Sorrow] {
			for value in [0.1, 0.9] {
				let packet = BlendShape::new(shape, value).into_osc_packet();
				let parsed_packet = &parse(packet)?[0];
				match parsed_packet {
					Message::BlendShape(blend) => {
						assert_eq!(blend.key.parse::<StandardVRMBlendShape>().unwrap(), shape);
						assert_relative_eq!(blend.value, value);
					}
					_ => panic!()
				}
			}
		}

		let packet = ApplyBlendShapes.into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::ApplyBlendShapes => (),
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_parse_state() -> Result<()> {
		let model_state = ModelState::Loaded;
		let calibration_state = CalibrationState::Calibrating;
		let calibration_mode = CalibrationMode::MixedRealityHand;
		let tracking_state = TrackingState::Poor;

		let packet = State::new(model_state).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::State(state) => {
				assert_eq!(state.model_state, model_state);
				assert!(state.calibration_state.is_none());
				assert!(state.tracking_state.is_none());
			}
			_ => panic!()
		}

		let packet = State::new_calibration(model_state, calibration_mode, calibration_state).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::State(state) => {
				assert_eq!(state.model_state, model_state);
				let calibration = state.calibration_state.unwrap();
				assert_eq!(calibration.0, calibration_mode);
				assert_eq!(calibration.1, calibration_state);
				assert!(state.tracking_state.is_none());
			}
			_ => panic!()
		}

		let packet = State::new_tracking(model_state, calibration_mode, calibration_state, tracking_state).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::State(state) => {
				assert_eq!(state.model_state, model_state);
				let calibration = state.calibration_state.unwrap();
				assert_eq!(calibration.0, calibration_mode);
				assert_eq!(calibration.1, calibration_state);
				assert_eq!(state.tracking_state.unwrap(), tracking_state);
			}
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_parse_time() -> Result<()> {
		let time_val = 7.0;

		let packet = Time::new(time_val).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			Message::Time(time) => {
				assert_relative_eq!(time.0, time_val);
			}
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_ignore_extra_args() -> Result<()> {
		assert!(
			parse(OscPacket::Message(OscMessage {
				addr: "/VMC/Ext/T".to_string(),
				args: vec![OscType::Float(7.0), OscType::String("hello".to_string())]
			}))
			.is_ok()
		);
		Ok(())
	}
}
