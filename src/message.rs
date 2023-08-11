use std::{str::FromStr, time::Instant};

use nalgebra::{Quaternion, Scale3, UnitQuaternion, Vector3};
use once_cell::sync::Lazy;

use crate::{osc::OSCMessage, IntoOSCMessage, OSCPacket, OSCType, VMCError, VMCResult};

#[derive(Debug, Clone, PartialEq)]
pub struct RootTransform {
	pub position: Vector3<f32>,
	pub rotation: UnitQuaternion<f32>,
	pub scale: Option<Scale3<f32>>,
	pub offset: Option<Vector3<f32>>
}

impl RootTransform {
	pub fn new(position: impl Into<Vector3<f32>>, rotation: UnitQuaternion<f32>) -> Self {
		Self {
			position: position.into(),
			rotation,
			scale: None,
			offset: None
		}
	}

	pub fn new_mr(position: impl Into<Vector3<f32>>, rotation: UnitQuaternion<f32>, scale: Scale3<f32>, offset: Vector3<f32>) -> Self {
		let rotation = rotation.slerp(&rotation, 1.0);
		Self {
			position: position.into(),
			rotation,
			scale: Some(scale),
			offset: Some(offset)
		}
	}
}

impl IntoOSCMessage for RootTransform {
	fn into_osc_message(self) -> crate::osc::OSCMessage {
		let rotation = self.rotation.as_ref();
		let mut args: Vec<OSCType> = vec![
			"root".into(),
			self.position.x.into(),
			self.position.y.into(),
			self.position.z.into(),
			rotation.coords.x.into(),
			rotation.coords.y.into(),
			rotation.coords.z.into(),
			rotation.coords.w.into(),
		];
		if let (Some(scale), Some(offset)) = (self.scale.as_ref(), self.offset.as_ref()) {
			args.extend([scale.x.into(), scale.y.into(), scale.z.into()]);
			args.extend([offset.x.into(), offset.y.into(), offset.z.into()]);
		}
		OSCMessage::new("/VMC/Ext/Root/Pos", args)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bone {
	Hips,
	LeftUpperLeg,
	RightUpperLeg,
	LeftLowerLeg,
	RightLowerLeg,
	LeftFoot,
	RightFoot,
	Pelvis,
	Spine,
	Chest,
	UpperChest,
	Neck,
	Head,
	LeftShoulder,
	RightShoulder,
	LeftUpperArm,
	RightUpperArm,
	LeftLowerArm,
	RightLowerArm,
	LeftHand,
	RightHand,
	LeftToes,
	RightToes,
	LeftEye,
	RightEye,
	Jaw,
	LeftThumbProximal,
	LeftThumbIntermediate,
	LeftThumbDistal,
	LeftIndexProximal,
	LeftIndexIntermediate,
	LeftIndexDistal,
	LeftMiddleProximal,
	LeftMiddleIntermediate,
	LeftMiddleDistal,
	LeftRingProximal,
	LeftRingIntermediate,
	LeftRingDistal,
	LeftLittleProximal,
	LeftLittleIntermediate,
	LeftLittleDistal,
	RightThumbProximal,
	RightThumbIntermediate,
	RightThumbDistal,
	RightIndexProximal,
	RightIndexIntermediate,
	RightIndexDistal,
	RightMiddleProximal,
	RightMiddleIntermediate,
	RightMiddleDistal,
	RightRingProximal,
	RightRingIntermediate,
	RightRingDistal,
	RightLittleProximal,
	RightLittleIntermediate,
	RightLittleDistal
}

impl AsRef<str> for Bone {
	fn as_ref(&self) -> &'static str {
		match self {
			Bone::Hips => "Hips",
			Bone::LeftUpperLeg => "LeftUpperLeg",
			Bone::RightUpperLeg => "RightUpperLeg",
			Bone::LeftLowerLeg => "LeftLowerLeg",
			Bone::RightLowerLeg => "RightLowerLeg",
			Bone::LeftFoot => "LeftFoot",
			Bone::RightFoot => "RightFoot",
			Bone::Pelvis => "Pelvis",
			Bone::Spine => "Spine",
			Bone::Chest => "Chest",
			Bone::UpperChest => "UpperChest",
			Bone::Neck => "Neck",
			Bone::Head => "Head",
			Bone::LeftShoulder => "LeftShoulder",
			Bone::RightShoulder => "RightShoulder",
			Bone::LeftUpperArm => "LeftUpperArm",
			Bone::RightUpperArm => "RightUpperArm",
			Bone::LeftLowerArm => "LeftLowerArm",
			Bone::RightLowerArm => "RightLowerArm",
			Bone::LeftHand => "LeftHand",
			Bone::RightHand => "RightHand",
			Bone::LeftToes => "LeftToes",
			Bone::RightToes => "RightToes",
			Bone::LeftEye => "LeftEye",
			Bone::RightEye => "RightEye",
			Bone::Jaw => "Jaw",
			Bone::LeftThumbProximal => "LeftThumbProximal",
			Bone::LeftThumbIntermediate => "LeftThumbIntermediate",
			Bone::LeftThumbDistal => "LeftThumbDistal",
			Bone::LeftIndexProximal => "LeftIndexProximal",
			Bone::LeftIndexIntermediate => "LeftIndexIntermediate",
			Bone::LeftIndexDistal => "LeftIndexDistal",
			Bone::LeftMiddleProximal => "LeftMiddleProximal",
			Bone::LeftMiddleIntermediate => "LeftMiddleIntermediate",
			Bone::LeftMiddleDistal => "LeftMiddleDistal",
			Bone::LeftRingProximal => "LeftRingProximal",
			Bone::LeftRingIntermediate => "LeftRingIntermediate",
			Bone::LeftRingDistal => "LeftRingDistal",
			Bone::LeftLittleProximal => "LeftLittleProximal",
			Bone::LeftLittleIntermediate => "LeftLittleIntermediate",
			Bone::LeftLittleDistal => "LeftLittleDistal",
			Bone::RightThumbProximal => "RightThumbProximal",
			Bone::RightThumbIntermediate => "RightThumbIntermediate",
			Bone::RightThumbDistal => "RightThumbDistal",
			Bone::RightIndexProximal => "RightIndexProximal",
			Bone::RightIndexIntermediate => "RightIndexIntermediate",
			Bone::RightIndexDistal => "RightIndexDistal",
			Bone::RightMiddleProximal => "RightMiddleProximal",
			Bone::RightMiddleIntermediate => "RightMiddleIntermediate",
			Bone::RightMiddleDistal => "RightMiddleDistal",
			Bone::RightRingProximal => "RightRingProximal",
			Bone::RightRingIntermediate => "RightRingIntermediate",
			Bone::RightRingDistal => "RightRingDistal",
			Bone::RightLittleProximal => "RightLittleProximal",
			Bone::RightLittleIntermediate => "RightLittleIntermediate",
			Bone::RightLittleDistal => "RightLittleDistal"
		}
	}
}

impl ToString for Bone {
	fn to_string(&self) -> String {
		self.as_ref().to_owned()
	}
}

impl FromStr for Bone {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Hips" => Ok(Bone::Hips),
			"LeftUpperLeg" => Ok(Bone::LeftUpperLeg),
			"RightUpperLeg" => Ok(Bone::RightUpperLeg),
			"LeftLowerLeg" => Ok(Bone::LeftLowerLeg),
			"RightLowerLeg" => Ok(Bone::RightLowerLeg),
			"LeftFoot" => Ok(Bone::LeftFoot),
			"RightFoot" => Ok(Bone::RightFoot),
			"Pelvis" => Ok(Bone::Pelvis),
			"Spine" => Ok(Bone::Spine),
			"Chest" => Ok(Bone::Chest),
			"UpperChest" => Ok(Bone::UpperChest),
			"Neck" => Ok(Bone::Neck),
			"Head" => Ok(Bone::Head),
			"LeftShoulder" => Ok(Bone::LeftShoulder),
			"RightShoulder" => Ok(Bone::RightShoulder),
			"LeftUpperArm" => Ok(Bone::LeftUpperArm),
			"RightUpperArm" => Ok(Bone::RightUpperArm),
			"LeftLowerArm" => Ok(Bone::LeftLowerArm),
			"RightLowerArm" => Ok(Bone::RightLowerArm),
			"LeftHand" => Ok(Bone::LeftHand),
			"RightHand" => Ok(Bone::RightHand),
			"LeftToes" => Ok(Bone::LeftToes),
			"RightToes" => Ok(Bone::RightToes),
			"LeftEye" => Ok(Bone::LeftEye),
			"RightEye" => Ok(Bone::RightEye),
			"Jaw" => Ok(Bone::Jaw),
			"LeftThumbProximal" => Ok(Bone::LeftThumbProximal),
			"LeftThumbIntermediate" => Ok(Bone::LeftThumbIntermediate),
			"LeftThumbDistal" => Ok(Bone::LeftThumbDistal),
			"LeftIndexProximal" => Ok(Bone::LeftIndexProximal),
			"LeftIndexIntermediate" => Ok(Bone::LeftIndexIntermediate),
			"LeftIndexDistal" => Ok(Bone::LeftIndexDistal),
			"LeftMiddleProximal" => Ok(Bone::LeftMiddleProximal),
			"LeftMiddleIntermediate" => Ok(Bone::LeftMiddleIntermediate),
			"LeftMiddleDistal" => Ok(Bone::LeftMiddleDistal),
			"LeftRingProximal" => Ok(Bone::LeftRingProximal),
			"LeftRingIntermediate" => Ok(Bone::LeftRingIntermediate),
			"LeftRingDistal" => Ok(Bone::LeftRingDistal),
			"LeftLittleProximal" => Ok(Bone::LeftLittleProximal),
			"LeftLittleIntermediate" => Ok(Bone::LeftLittleIntermediate),
			"LeftLittleDistal" => Ok(Bone::LeftLittleDistal),
			"RightThumbProximal" => Ok(Bone::RightThumbProximal),
			"RightThumbIntermediate" => Ok(Bone::RightThumbIntermediate),
			"RightThumbDistal" => Ok(Bone::RightThumbDistal),
			"RightIndexProximal" => Ok(Bone::RightIndexProximal),
			"RightIndexIntermediate" => Ok(Bone::RightIndexIntermediate),
			"RightIndexDistal" => Ok(Bone::RightIndexDistal),
			"RightMiddleProximal" => Ok(Bone::RightMiddleProximal),
			"RightMiddleIntermediate" => Ok(Bone::RightMiddleIntermediate),
			"RightMiddleDistal" => Ok(Bone::RightMiddleDistal),
			"RightRingProximal" => Ok(Bone::RightRingProximal),
			"RightRingIntermediate" => Ok(Bone::RightRingIntermediate),
			"RightRingDistal" => Ok(Bone::RightRingDistal),
			"RightLittleProximal" => Ok(Bone::RightLittleProximal),
			"RightLittleIntermediate" => Ok(Bone::RightLittleIntermediate),
			"RightLittleDistal" => Ok(Bone::RightLittleDistal),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoneTransform {
	pub bone: Bone,
	pub position: Vector3<f32>,
	pub rotation: UnitQuaternion<f32>,
	pub scale: Option<Scale3<f32>>,
	pub offset: Option<Vector3<f32>>
}

impl BoneTransform {
	pub fn new(bone: Bone, position: impl Into<Vector3<f32>>, rotation: UnitQuaternion<f32>) -> Self {
		Self {
			bone,
			position: position.into(),
			rotation,
			scale: None,
			offset: None
		}
	}

	pub fn new_mr(bone: Bone, position: impl Into<Vector3<f32>>, rotation: UnitQuaternion<f32>, scale: Scale3<f32>, offset: Vector3<f32>) -> Self {
		let rotation = rotation.slerp(&rotation, 1.0);
		Self {
			bone,
			position: position.into(),
			rotation,
			scale: Some(scale),
			offset: Some(offset)
		}
	}
}

impl IntoOSCMessage for BoneTransform {
	fn into_osc_message(self) -> crate::osc::OSCMessage {
		let rotation = self.rotation.as_ref();
		let mut args: Vec<OSCType> = vec![
			self.bone.as_ref().into(),
			self.position.x.into(),
			self.position.y.into(),
			self.position.z.into(),
			rotation.coords.x.into(),
			rotation.coords.y.into(),
			rotation.coords.z.into(),
			rotation.coords.w.into(),
		];
		if let (Some(scale), Some(offset)) = (self.scale.as_ref(), self.offset.as_ref()) {
			args.extend([scale.x.into(), scale.y.into(), scale.z.into()]);
			args.extend([offset.x.into(), offset.y.into(), offset.z.into()]);
		}
		OSCMessage::new("/VMC/Ext/Bone/Pos", args)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
	HMD,
	Controller,
	Tracker
}

impl AsRef<str> for DeviceType {
	fn as_ref(&self) -> &str {
		match self {
			DeviceType::HMD => "Hmd",
			DeviceType::Controller => "Con",
			DeviceType::Tracker => "Tra"
		}
	}
}

impl ToString for DeviceType {
	fn to_string(&self) -> String {
		self.as_ref().to_owned()
	}
}

impl FromStr for DeviceType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Hmd" => Ok(DeviceType::HMD),
			"Con" => Ok(DeviceType::Controller),
			"Tra" => Ok(DeviceType::Tracker),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceTransform {
	pub device: DeviceType,
	pub joint: String,
	pub position: Vector3<f32>,
	pub rotation: UnitQuaternion<f32>,
	pub local: bool
}

impl DeviceTransform {
	pub fn new(device: DeviceType, joint: impl ToString, position: impl Into<Vector3<f32>>, rotation: UnitQuaternion<f32>, local: bool) -> Self {
		Self {
			device,
			joint: joint.to_string(),
			position: position.into(),
			rotation,
			local
		}
	}
}

impl IntoOSCMessage for DeviceTransform {
	fn into_osc_message(self) -> crate::osc::OSCMessage {
		let rotation = self.rotation.as_ref();
		OSCMessage::new(
			format!("/VMC/Ext/{}/Pos{}", self.device.as_ref(), if self.local { "/Local" } else { "" }),
			(self.joint, self.position.x, self.position.y, self.position.z, rotation.coords.x, rotation.coords.y, rotation.coords.z, rotation.coords.w)
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardVRMBlendShape {
	Neutral,
	A,
	I,
	U,
	E,
	O,
	Blink,
	Joy,
	Angry,
	Sorrow,
	Fun,
	LookUp,
	LookDown,
	LookLeft,
	LookRight,
	BlinkL,
	BlinkR
}

impl AsRef<str> for StandardVRMBlendShape {
	fn as_ref(&self) -> &str {
		match self {
			StandardVRMBlendShape::Neutral => "Neutral",
			StandardVRMBlendShape::A => "A",
			StandardVRMBlendShape::I => "I",
			StandardVRMBlendShape::U => "U",
			StandardVRMBlendShape::E => "E",
			StandardVRMBlendShape::O => "O",
			StandardVRMBlendShape::Blink => "Blink",
			StandardVRMBlendShape::Joy => "Joy",
			StandardVRMBlendShape::Angry => "Angry",
			StandardVRMBlendShape::Sorrow => "Sorrow",
			StandardVRMBlendShape::Fun => "Fun",
			StandardVRMBlendShape::LookUp => "LookUp",
			StandardVRMBlendShape::LookDown => "LookDown",
			StandardVRMBlendShape::LookLeft => "LookLeft",
			StandardVRMBlendShape::LookRight => "LookRight",
			StandardVRMBlendShape::BlinkL => "Blink_L",
			StandardVRMBlendShape::BlinkR => "Blink_R"
		}
	}
}

impl ToString for StandardVRMBlendShape {
	fn to_string(&self) -> String {
		self.as_ref().to_owned()
	}
}

impl FromStr for StandardVRMBlendShape {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Neutral" => Ok(StandardVRMBlendShape::Neutral),
			"A" => Ok(StandardVRMBlendShape::A),
			"I" => Ok(StandardVRMBlendShape::I),
			"U" => Ok(StandardVRMBlendShape::U),
			"E" => Ok(StandardVRMBlendShape::E),
			"O" => Ok(StandardVRMBlendShape::O),
			"Blink" => Ok(StandardVRMBlendShape::Blink),
			"Joy" => Ok(StandardVRMBlendShape::Joy),
			"Angry" => Ok(StandardVRMBlendShape::Angry),
			"Sorrow" => Ok(StandardVRMBlendShape::Sorrow),
			"Fun" => Ok(StandardVRMBlendShape::Fun),
			"LookUp" => Ok(StandardVRMBlendShape::LookUp),
			"LookDown" => Ok(StandardVRMBlendShape::LookDown),
			"LookLeft" => Ok(StandardVRMBlendShape::LookLeft),
			"LookRight" => Ok(StandardVRMBlendShape::LookRight),
			"Blink_L" => Ok(StandardVRMBlendShape::BlinkL),
			"Blink_R" => Ok(StandardVRMBlendShape::BlinkR),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlendShape {
	pub key: String,
	pub value: f32
}

impl BlendShape {
	pub fn new(key: impl ToString, value: f32) -> Self {
		Self { key: key.to_string(), value }
	}
}

impl IntoOSCMessage for BlendShape {
	fn into_osc_message(self) -> OSCMessage {
		OSCMessage::new("/VMC/Ext/Blend/Val", (self.key, self.value))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyBlendShapes;

impl IntoOSCMessage for ApplyBlendShapes {
	fn into_osc_message(self) -> OSCMessage {
		OSCMessage::new("/VMC/Ext/Blend/Apply", ())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ModelState {
	NotLoaded = 0,
	Loaded = 1
}

impl From<ModelState> for OSCType {
	fn from(value: ModelState) -> Self {
		OSCType::Int(value as i32)
	}
}

impl TryFrom<i32> for ModelState {
	type Error = i32;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(ModelState::NotLoaded),
			1 => Ok(ModelState::Loaded),
			x => Err(x)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CalibrationState {
	Uncalibrated = 0,
	WaitingForCalibration = 1,
	Calibrating = 2,
	Calibrated = 3
}

impl From<CalibrationState> for OSCType {
	fn from(value: CalibrationState) -> Self {
		OSCType::Int(value as i32)
	}
}

impl TryFrom<i32> for CalibrationState {
	type Error = i32;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(CalibrationState::Uncalibrated),
			1 => Ok(CalibrationState::WaitingForCalibration),
			2 => Ok(CalibrationState::Calibrating),
			3 => Ok(CalibrationState::Calibrated),
			x => Err(x)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CalibrationMode {
	Normal = 0,
	MixedRealityHand = 1,
	MixedRealityFloor = 2
}

impl From<CalibrationMode> for OSCType {
	fn from(value: CalibrationMode) -> Self {
		OSCType::Int(value as i32)
	}
}

impl TryFrom<i32> for CalibrationMode {
	type Error = i32;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(CalibrationMode::Normal),
			1 => Ok(CalibrationMode::MixedRealityHand),
			2 => Ok(CalibrationMode::MixedRealityFloor),
			x => Err(x)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TrackingState {
	Poor = 0,
	Good = 1
}

impl From<TrackingState> for OSCType {
	fn from(value: TrackingState) -> Self {
		OSCType::Int(value as i32)
	}
}

impl TryFrom<i32> for TrackingState {
	type Error = i32;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(TrackingState::Poor),
			1 => Ok(TrackingState::Good),
			x => Err(x)
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
	pub model_state: ModelState,
	pub calibration_state: Option<(CalibrationMode, CalibrationState)>,
	pub tracking_state: Option<TrackingState>
}

impl State {
	pub fn new(model_state: ModelState) -> State {
		Self {
			model_state,
			calibration_state: None,
			tracking_state: None
		}
	}

	pub fn new_calibration(model_state: ModelState, calibration_mode: CalibrationMode, calibration_state: CalibrationState) -> State {
		Self {
			model_state,
			calibration_state: Some((calibration_mode, calibration_state)),
			tracking_state: None
		}
	}

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
	fn into_osc_message(self) -> OSCMessage {
		let mut args: Vec<OSCType> = vec![self.model_state.into()];
		if let Some((calibration_mode, calibration_state)) = self.calibration_state {
			args.extend([calibration_state.into(), calibration_mode.into()]);
			if let Some(tracking_state) = self.tracking_state {
				args.push(tracking_state.into());
			}
		}
		OSCMessage::new("/VMC/Ext/OK", args)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time(pub f32);

impl Time {
	pub fn new(timestamp: f32) -> Self {
		Self(timestamp)
	}

	pub fn elapsed() -> Self {
		static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);
		Self(EPOCH.elapsed().as_secs_f32())
	}
}

impl IntoOSCMessage for Time {
	fn into_osc_message(self) -> OSCMessage {
		OSCMessage::new("/VMC/Ext/T", (self.0,))
	}
}

#[derive(Debug, Clone)]
pub enum VMCMessage {
	RootTransform(RootTransform),
	DeviceTransform(DeviceTransform),
	BoneTransform(BoneTransform),
	BlendShape(BlendShape),
	ApplyBlendShapes,
	State(State),
	Time(Time)
}

impl IntoOSCMessage for VMCMessage {
	fn into_osc_message(self) -> OSCMessage {
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

impl From<RootTransform> for VMCMessage {
	fn from(value: RootTransform) -> Self {
		Self::RootTransform(value)
	}
}
impl From<DeviceTransform> for VMCMessage {
	fn from(value: DeviceTransform) -> Self {
		Self::DeviceTransform(value)
	}
}
impl From<BoneTransform> for VMCMessage {
	fn from(value: BoneTransform) -> Self {
		Self::BoneTransform(value)
	}
}
impl From<BlendShape> for VMCMessage {
	fn from(value: BlendShape) -> Self {
		Self::BlendShape(value)
	}
}
impl From<ApplyBlendShapes> for VMCMessage {
	fn from(_value: ApplyBlendShapes) -> Self {
		Self::ApplyBlendShapes
	}
}
impl From<State> for VMCMessage {
	fn from(value: State) -> Self {
		Self::State(value)
	}
}
impl From<Time> for VMCMessage {
	fn from(value: Time) -> Self {
		Self::Time(value)
	}
}

fn flatten_packet(packet: OSCPacket) -> Vec<OSCMessage> {
	match packet {
		OSCPacket::Bundle(bundle) => bundle.content.into_iter().flat_map(flatten_packet).collect(),
		OSCPacket::Message(message) => vec![message]
	}
}

pub fn parse(osc_packet: OSCPacket) -> VMCResult<Vec<VMCMessage>> {
	let messages = flatten_packet(osc_packet);
	messages
		.into_iter()
		.map(|msg| match msg.as_tuple() {
			(
				"/VMC/Ext/Root/Pos",
				&[
					OSCType::String(_),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w)
				]
			) => Ok(VMCMessage::RootTransform(RootTransform::new(
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z))
			))),
			(
				"/VMC/Ext/Root/Pos",
				&[
					OSCType::String(_),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					OSCType::Float(s_x),
					OSCType::Float(s_y),
					OSCType::Float(s_z),
					OSCType::Float(o_x),
					OSCType::Float(o_y),
					OSCType::Float(o_z),
					..
				]
			) => Ok(VMCMessage::RootTransform(RootTransform::new_mr(
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				Scale3::new(s_x, s_y, s_z),
				Vector3::new(o_x, o_y, o_z)
			))),
			(
				"/VMC/Ext/Bone/Pos",
				&[
					OSCType::String(ref bone),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w)
				]
			) => Ok(VMCMessage::BoneTransform(BoneTransform::new(
				Bone::from_str(bone).map_err(|_| VMCError::UnknownBone(bone.to_string()))?,
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z))
			))),
			(
				"/VMC/Ext/Bone/Pos",
				&[
					OSCType::String(ref bone),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					OSCType::Float(s_x),
					OSCType::Float(s_y),
					OSCType::Float(s_z),
					OSCType::Float(o_x),
					OSCType::Float(o_y),
					OSCType::Float(o_z),
					..
				]
			) => Ok(VMCMessage::BoneTransform(BoneTransform::new_mr(
				Bone::from_str(bone).map_err(|_| VMCError::UnknownBone(bone.to_string()))?,
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				Scale3::new(s_x, s_y, s_z),
				Vector3::new(o_x, o_y, o_z)
			))),
			(
				"/VMC/Ext/Hmd/Pos",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::HMD,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				false
			))),
			(
				"/VMC/Ext/Hmd/Pos/Local",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::HMD,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				true
			))),
			(
				"/VMC/Ext/Con/Pos",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::Controller,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				false
			))),
			(
				"/VMC/Ext/Con/Pos/Local",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::Controller,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				true
			))),
			(
				"/VMC/Ext/Tra/Pos",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::Tracker,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				false
			))),
			(
				"/VMC/Ext/Tra/Pos/Local",
				&[
					OSCType::String(ref joint),
					OSCType::Float(p_x),
					OSCType::Float(p_y),
					OSCType::Float(p_z),
					OSCType::Float(r_x),
					OSCType::Float(r_y),
					OSCType::Float(r_z),
					OSCType::Float(r_w),
					..
				]
			) => Ok(VMCMessage::DeviceTransform(DeviceTransform::new(
				DeviceType::Tracker,
				joint.to_owned(),
				Vector3::new(p_x, p_y, p_z),
				UnitQuaternion::new_unchecked(Quaternion::new(r_w, r_x, r_y, r_z)),
				true
			))),
			("/VMC/Ext/Blend/Val", &[OSCType::String(ref shape), OSCType::Float(val), ..]) => Ok(VMCMessage::BlendShape(BlendShape::new(shape, val))),
			("/VMC/Ext/Blend/Apply", &[..]) => Ok(VMCMessage::ApplyBlendShapes),
			("/VMC/Ext/OK", &[OSCType::Int(model_state)]) => Ok(VMCMessage::State(State::new(model_state.try_into().map_err(VMCError::UnknownModelState)?))),
			("/VMC/Ext/OK", &[OSCType::Int(model_state), OSCType::Int(calibration_state), OSCType::Int(calibration_mode)]) => {
				Ok(VMCMessage::State(State::new_calibration(
					model_state.try_into().map_err(VMCError::UnknownModelState)?,
					calibration_mode.try_into().map_err(VMCError::UnknownCalibrationMode)?,
					calibration_state.try_into().map_err(VMCError::UnknownCalibrationState)?
				)))
			}
			(
				"/VMC/Ext/OK",
				&[
					OSCType::Int(model_state),
					OSCType::Int(calibration_state),
					OSCType::Int(calibration_mode),
					OSCType::Int(tracking_state),
					..
				]
			) => Ok(VMCMessage::State(State::new_tracking(
				model_state.try_into().map_err(VMCError::UnknownModelState)?,
				calibration_mode.try_into().map_err(VMCError::UnknownCalibrationMode)?,
				calibration_state.try_into().map_err(VMCError::UnknownCalibrationState)?,
				tracking_state.try_into().map_err(VMCError::UnknownTrackingState)?
			))),
			("/VMC/Ext/T", &[OSCType::Float(time), ..]) => Ok(VMCMessage::Time(Time::new(time))),
			(addr, args) => Err(VMCError::UnimplementedMessage(addr.to_owned(), args.to_owned()))
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use approx::assert_relative_eq;

	use super::*;
	use crate::IntoOSCPacket;

	#[test]
	fn test_parse_root_transform() -> VMCResult<()> {
		let position = Vector3::new(0.5, 0.2, -0.4);
		let rotation = UnitQuaternion::new_normalize(Quaternion::new(1.0, 2.0, 3.0, 4.0));
		let scale = Scale3::new(0.8, 1.0, 0.3);
		let offset = Vector3::new(-0.1, 0.12, -0.3);

		let packet = RootTransform::new(position, rotation).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			VMCMessage::RootTransform(transform) => {
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
			VMCMessage::RootTransform(transform) => {
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
	fn test_parse_bone_transform() -> VMCResult<()> {
		let position = Vector3::new(0.5, 0.2, -0.4);
		let rotation = UnitQuaternion::new_normalize(Quaternion::new(1.0, 2.0, 3.0, 4.0));
		let scale = Scale3::new(0.8, 1.0, 0.3);
		let offset = Vector3::new(-0.1, 0.12, -0.3);

		for bone in [Bone::Chest, Bone::RightEye, Bone::LeftIndexDistal, Bone::Spine, Bone::RightHand] {
			let packet = BoneTransform::new(bone, position, rotation).into_osc_packet();
			let parsed_packet = &parse(packet)?[0];
			match parsed_packet {
				VMCMessage::BoneTransform(transform) => {
					assert_eq!(transform.bone, bone);
					assert_relative_eq!(transform.position, position);
					assert_relative_eq!(transform.rotation, rotation);
					assert!(transform.scale.is_none());
					assert!(transform.offset.is_none());
				}
				_ => panic!()
			}

			let packet = BoneTransform::new_mr(bone, position, rotation, scale, offset).into_osc_packet();
			let parsed_packet = &parse(packet)?[0];
			match parsed_packet {
				VMCMessage::BoneTransform(transform) => {
					assert_eq!(transform.bone, bone);
					assert_relative_eq!(transform.position, position);
					assert_relative_eq!(transform.rotation, rotation);
					assert_relative_eq!(transform.scale.unwrap(), scale);
					assert_relative_eq!(transform.offset.unwrap(), offset);
				}
				_ => panic!()
			}
		}

		Ok(())
	}

	#[test]
	fn test_parse_device_transform() -> VMCResult<()> {
		let position = Vector3::new(0.5, 0.2, -0.4);
		let rotation = UnitQuaternion::new_normalize(Quaternion::new(1.0, 2.0, 3.0, 4.0));

		for device in [DeviceType::HMD, DeviceType::Controller, DeviceType::Tracker] {
			for joint in ["Head", "LeftHand"] {
				for locality in [true, false] {
					let packet = DeviceTransform::new(device, joint, position, rotation, locality).into_osc_packet();
					let parsed_packet = &parse(packet)?[0];
					match parsed_packet {
						VMCMessage::DeviceTransform(transform) => {
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
	fn test_parse_blend_shape() -> VMCResult<()> {
		for shape in [StandardVRMBlendShape::A, StandardVRMBlendShape::LookRight, StandardVRMBlendShape::Sorrow] {
			for value in [0.1, 0.9] {
				let packet = BlendShape::new(shape, value).into_osc_packet();
				let parsed_packet = &parse(packet)?[0];
				match parsed_packet {
					VMCMessage::BlendShape(blend) => {
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
			VMCMessage::ApplyBlendShapes => (),
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_parse_state() -> VMCResult<()> {
		let model_state = ModelState::Loaded;
		let calibration_state = CalibrationState::Calibrating;
		let calibration_mode = CalibrationMode::MixedRealityHand;
		let tracking_state = TrackingState::Poor;

		let packet = State::new(model_state).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			VMCMessage::State(state) => {
				assert_eq!(state.model_state, model_state);
				assert!(state.calibration_state.is_none());
				assert!(state.tracking_state.is_none());
			}
			_ => panic!()
		}

		let packet = State::new_calibration(model_state, calibration_mode, calibration_state).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			VMCMessage::State(state) => {
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
			VMCMessage::State(state) => {
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
	fn test_parse_time() -> VMCResult<()> {
		let time_val = 7.0;

		let packet = Time::new(time_val).into_osc_packet();
		let parsed_packet = &parse(packet)?[0];
		match parsed_packet {
			VMCMessage::Time(time) => {
				assert_relative_eq!(time.0, time_val);
			}
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn test_ignore_extra_args() -> VMCResult<()> {
		assert!(parse(OSCPacket::Message(OSCMessage::new("/VMC/Ext/T", (7.0_f32, "hello")))).is_ok());
		Ok(())
	}
}
