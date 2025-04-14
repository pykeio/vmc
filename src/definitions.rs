use std::str::FromStr;

use rosc::OscType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quat {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

impl Quat {
	pub const fn from_array(a: [f32; 4]) -> Self {
		Self::from_xyzw(a[0], a[1], a[2], a[3])
	}

	pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x, y, z, w }
	}
}

impl From<[f32; 4]> for Quat {
	fn from(value: [f32; 4]) -> Self {
		Self::from_array(value)
	}
}

#[cfg(test)]
impl approx::AbsDiffEq for Quat {
	type Epsilon = <f32 as approx::AbsDiffEq>::Epsilon;

	fn default_epsilon() -> Self::Epsilon {
		f32::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		f32::abs_diff_eq(&self.x, &other.x, epsilon)
			&& f32::abs_diff_eq(&self.y, &other.y, epsilon)
			&& f32::abs_diff_eq(&self.z, &other.z, epsilon)
			&& f32::abs_diff_eq(&self.w, &other.w, epsilon)
	}
}

#[cfg(test)]
impl approx::RelativeEq for Quat {
	fn default_max_relative() -> Self::Epsilon {
		f32::default_max_relative()
	}

	fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
		f32::relative_eq(&self.x, &other.x, epsilon, max_relative)
			&& f32::relative_eq(&self.y, &other.y, epsilon, max_relative)
			&& f32::relative_eq(&self.z, &other.z, epsilon, max_relative)
			&& f32::relative_eq(&self.w, &other.w, epsilon, max_relative)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vec3 {
	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub const fn from_array(a: [f32; 3]) -> Self {
		Self::new(a[0], a[1], a[2])
	}
}

impl From<[f32; 3]> for Vec3 {
	fn from(value: [f32; 3]) -> Self {
		Self::from_array(value)
	}
}

#[cfg(test)]
impl approx::AbsDiffEq for Vec3 {
	type Epsilon = <f32 as approx::AbsDiffEq>::Epsilon;

	fn default_epsilon() -> Self::Epsilon {
		f32::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		f32::abs_diff_eq(&self.x, &other.x, epsilon) && f32::abs_diff_eq(&self.y, &other.y, epsilon) && f32::abs_diff_eq(&self.z, &other.z, epsilon)
	}
}

#[cfg(test)]
impl approx::RelativeEq for Vec3 {
	fn default_max_relative() -> Self::Epsilon {
		f32::default_max_relative()
	}

	fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
		f32::relative_eq(&self.x, &other.x, epsilon, max_relative)
			&& f32::relative_eq(&self.y, &other.y, epsilon, max_relative)
			&& f32::relative_eq(&self.z, &other.z, epsilon, max_relative)
	}
}

/// Standard bones used by VRM 0.x.
///
/// <https://github.com/vrm-c/vrm-specification/blob/master/specification/0.0/README.md#defined-bones>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StandardVRM0Bone {
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

impl AsRef<str> for StandardVRM0Bone {
	fn as_ref(&self) -> &'static str {
		match self {
			StandardVRM0Bone::Hips => "Hips",
			StandardVRM0Bone::LeftUpperLeg => "LeftUpperLeg",
			StandardVRM0Bone::RightUpperLeg => "RightUpperLeg",
			StandardVRM0Bone::LeftLowerLeg => "LeftLowerLeg",
			StandardVRM0Bone::RightLowerLeg => "RightLowerLeg",
			StandardVRM0Bone::LeftFoot => "LeftFoot",
			StandardVRM0Bone::RightFoot => "RightFoot",
			StandardVRM0Bone::Pelvis => "Pelvis",
			StandardVRM0Bone::Spine => "Spine",
			StandardVRM0Bone::Chest => "Chest",
			StandardVRM0Bone::UpperChest => "UpperChest",
			StandardVRM0Bone::Neck => "Neck",
			StandardVRM0Bone::Head => "Head",
			StandardVRM0Bone::LeftShoulder => "LeftShoulder",
			StandardVRM0Bone::RightShoulder => "RightShoulder",
			StandardVRM0Bone::LeftUpperArm => "LeftUpperArm",
			StandardVRM0Bone::RightUpperArm => "RightUpperArm",
			StandardVRM0Bone::LeftLowerArm => "LeftLowerArm",
			StandardVRM0Bone::RightLowerArm => "RightLowerArm",
			StandardVRM0Bone::LeftHand => "LeftHand",
			StandardVRM0Bone::RightHand => "RightHand",
			StandardVRM0Bone::LeftToes => "LeftToes",
			StandardVRM0Bone::RightToes => "RightToes",
			StandardVRM0Bone::LeftEye => "LeftEye",
			StandardVRM0Bone::RightEye => "RightEye",
			StandardVRM0Bone::Jaw => "Jaw",
			StandardVRM0Bone::LeftThumbProximal => "LeftThumbProximal",
			StandardVRM0Bone::LeftThumbIntermediate => "LeftThumbIntermediate",
			StandardVRM0Bone::LeftThumbDistal => "LeftThumbDistal",
			StandardVRM0Bone::LeftIndexProximal => "LeftIndexProximal",
			StandardVRM0Bone::LeftIndexIntermediate => "LeftIndexIntermediate",
			StandardVRM0Bone::LeftIndexDistal => "LeftIndexDistal",
			StandardVRM0Bone::LeftMiddleProximal => "LeftMiddleProximal",
			StandardVRM0Bone::LeftMiddleIntermediate => "LeftMiddleIntermediate",
			StandardVRM0Bone::LeftMiddleDistal => "LeftMiddleDistal",
			StandardVRM0Bone::LeftRingProximal => "LeftRingProximal",
			StandardVRM0Bone::LeftRingIntermediate => "LeftRingIntermediate",
			StandardVRM0Bone::LeftRingDistal => "LeftRingDistal",
			StandardVRM0Bone::LeftLittleProximal => "LeftLittleProximal",
			StandardVRM0Bone::LeftLittleIntermediate => "LeftLittleIntermediate",
			StandardVRM0Bone::LeftLittleDistal => "LeftLittleDistal",
			StandardVRM0Bone::RightThumbProximal => "RightThumbProximal",
			StandardVRM0Bone::RightThumbIntermediate => "RightThumbIntermediate",
			StandardVRM0Bone::RightThumbDistal => "RightThumbDistal",
			StandardVRM0Bone::RightIndexProximal => "RightIndexProximal",
			StandardVRM0Bone::RightIndexIntermediate => "RightIndexIntermediate",
			StandardVRM0Bone::RightIndexDistal => "RightIndexDistal",
			StandardVRM0Bone::RightMiddleProximal => "RightMiddleProximal",
			StandardVRM0Bone::RightMiddleIntermediate => "RightMiddleIntermediate",
			StandardVRM0Bone::RightMiddleDistal => "RightMiddleDistal",
			StandardVRM0Bone::RightRingProximal => "RightRingProximal",
			StandardVRM0Bone::RightRingIntermediate => "RightRingIntermediate",
			StandardVRM0Bone::RightRingDistal => "RightRingDistal",
			StandardVRM0Bone::RightLittleProximal => "RightLittleProximal",
			StandardVRM0Bone::RightLittleIntermediate => "RightLittleIntermediate",
			StandardVRM0Bone::RightLittleDistal => "RightLittleDistal"
		}
	}
}

impl ToString for StandardVRM0Bone {
	fn to_string(&self) -> String {
		self.as_ref().to_owned()
	}
}

impl FromStr for StandardVRM0Bone {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Hips" => Ok(StandardVRM0Bone::Hips),
			"LeftUpperLeg" => Ok(StandardVRM0Bone::LeftUpperLeg),
			"RightUpperLeg" => Ok(StandardVRM0Bone::RightUpperLeg),
			"LeftLowerLeg" => Ok(StandardVRM0Bone::LeftLowerLeg),
			"RightLowerLeg" => Ok(StandardVRM0Bone::RightLowerLeg),
			"LeftFoot" => Ok(StandardVRM0Bone::LeftFoot),
			"RightFoot" => Ok(StandardVRM0Bone::RightFoot),
			"Pelvis" => Ok(StandardVRM0Bone::Pelvis),
			"Spine" => Ok(StandardVRM0Bone::Spine),
			"Chest" => Ok(StandardVRM0Bone::Chest),
			"UpperChest" => Ok(StandardVRM0Bone::UpperChest),
			"Neck" => Ok(StandardVRM0Bone::Neck),
			"Head" => Ok(StandardVRM0Bone::Head),
			"LeftShoulder" => Ok(StandardVRM0Bone::LeftShoulder),
			"RightShoulder" => Ok(StandardVRM0Bone::RightShoulder),
			"LeftUpperArm" => Ok(StandardVRM0Bone::LeftUpperArm),
			"RightUpperArm" => Ok(StandardVRM0Bone::RightUpperArm),
			"LeftLowerArm" => Ok(StandardVRM0Bone::LeftLowerArm),
			"RightLowerArm" => Ok(StandardVRM0Bone::RightLowerArm),
			"LeftHand" => Ok(StandardVRM0Bone::LeftHand),
			"RightHand" => Ok(StandardVRM0Bone::RightHand),
			"LeftToes" => Ok(StandardVRM0Bone::LeftToes),
			"RightToes" => Ok(StandardVRM0Bone::RightToes),
			"LeftEye" => Ok(StandardVRM0Bone::LeftEye),
			"RightEye" => Ok(StandardVRM0Bone::RightEye),
			"Jaw" => Ok(StandardVRM0Bone::Jaw),
			"LeftThumbProximal" => Ok(StandardVRM0Bone::LeftThumbProximal),
			"LeftThumbIntermediate" => Ok(StandardVRM0Bone::LeftThumbIntermediate),
			"LeftThumbDistal" => Ok(StandardVRM0Bone::LeftThumbDistal),
			"LeftIndexProximal" => Ok(StandardVRM0Bone::LeftIndexProximal),
			"LeftIndexIntermediate" => Ok(StandardVRM0Bone::LeftIndexIntermediate),
			"LeftIndexDistal" => Ok(StandardVRM0Bone::LeftIndexDistal),
			"LeftMiddleProximal" => Ok(StandardVRM0Bone::LeftMiddleProximal),
			"LeftMiddleIntermediate" => Ok(StandardVRM0Bone::LeftMiddleIntermediate),
			"LeftMiddleDistal" => Ok(StandardVRM0Bone::LeftMiddleDistal),
			"LeftRingProximal" => Ok(StandardVRM0Bone::LeftRingProximal),
			"LeftRingIntermediate" => Ok(StandardVRM0Bone::LeftRingIntermediate),
			"LeftRingDistal" => Ok(StandardVRM0Bone::LeftRingDistal),
			"LeftLittleProximal" => Ok(StandardVRM0Bone::LeftLittleProximal),
			"LeftLittleIntermediate" => Ok(StandardVRM0Bone::LeftLittleIntermediate),
			"LeftLittleDistal" => Ok(StandardVRM0Bone::LeftLittleDistal),
			"RightThumbProximal" => Ok(StandardVRM0Bone::RightThumbProximal),
			"RightThumbIntermediate" => Ok(StandardVRM0Bone::RightThumbIntermediate),
			"RightThumbDistal" => Ok(StandardVRM0Bone::RightThumbDistal),
			"RightIndexProximal" => Ok(StandardVRM0Bone::RightIndexProximal),
			"RightIndexIntermediate" => Ok(StandardVRM0Bone::RightIndexIntermediate),
			"RightIndexDistal" => Ok(StandardVRM0Bone::RightIndexDistal),
			"RightMiddleProximal" => Ok(StandardVRM0Bone::RightMiddleProximal),
			"RightMiddleIntermediate" => Ok(StandardVRM0Bone::RightMiddleIntermediate),
			"RightMiddleDistal" => Ok(StandardVRM0Bone::RightMiddleDistal),
			"RightRingProximal" => Ok(StandardVRM0Bone::RightRingProximal),
			"RightRingIntermediate" => Ok(StandardVRM0Bone::RightRingIntermediate),
			"RightRingDistal" => Ok(StandardVRM0Bone::RightRingDistal),
			"RightLittleProximal" => Ok(StandardVRM0Bone::RightLittleProximal),
			"RightLittleIntermediate" => Ok(StandardVRM0Bone::RightLittleIntermediate),
			"RightLittleDistal" => Ok(StandardVRM0Bone::RightLittleDistal),
			_ => Err(())
		}
	}
}

impl PartialEq<&str> for StandardVRM0Bone {
	fn eq(&self, other: &&str) -> bool {
		StandardVRM0Bone::from_str(other).as_ref() == Ok(self)
	}
}
impl PartialEq<String> for StandardVRM0Bone {
	fn eq(&self, other: &String) -> bool {
		StandardVRM0Bone::from_str(other).as_ref() == Ok(self)
	}
}
impl PartialEq<StandardVRM0Bone> for &str {
	fn eq(&self, other: &StandardVRM0Bone) -> bool {
		StandardVRM0Bone::from_str(self).as_ref() == Ok(other)
	}
}
impl PartialEq<StandardVRM0Bone> for String {
	fn eq(&self, other: &StandardVRM0Bone) -> bool {
		StandardVRM0Bone::from_str(self).as_ref() == Ok(other)
	}
}

/// The type of device used in [`crate::DeviceTransform`] (HMD, controller, or independent tracker).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Standard blend shapes, in VRM 0.x format.
///
/// Senders using a 1.x avatar should map from 1.x expressions to 0.x blendshapes when sending VMC messages.
/// Receivers should be prepared to accept both 1.x and 0.x blend shapes:
/// <https://protocol.vmc.info/marionette-spec#vrm-blendshapeproxyvalue>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl PartialEq<&str> for StandardVRMBlendShape {
	fn eq(&self, other: &&str) -> bool {
		StandardVRMBlendShape::from_str(other).as_ref() == Ok(self)
	}
}
impl PartialEq<String> for StandardVRMBlendShape {
	fn eq(&self, other: &String) -> bool {
		StandardVRMBlendShape::from_str(other).as_ref() == Ok(self)
	}
}
impl PartialEq<StandardVRMBlendShape> for &str {
	fn eq(&self, other: &StandardVRMBlendShape) -> bool {
		StandardVRMBlendShape::from_str(self).as_ref() == Ok(other)
	}
}
impl PartialEq<StandardVRMBlendShape> for String {
	fn eq(&self, other: &StandardVRMBlendShape) -> bool {
		StandardVRMBlendShape::from_str(self).as_ref() == Ok(other)
	}
}

/// Loading state of the virtual avatar on the sender's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum ModelState {
	/// The model is not yet loaded or is currently loading.
	NotLoaded = 0,
	/// The model is loaded and tracking can commence.
	Loaded = 1
}

impl From<ModelState> for OscType {
	fn from(value: ModelState) -> Self {
		OscType::Int(value as i32)
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum CalibrationState {
	/// The sender has not yet calibrated tracking.
	Uncalibrated = 0,
	/// The sender is waiting for calibration to start (i.e. loading a tracking model)
	WaitingForCalibration = 1,
	/// The sender is currently calibrating tracking.
	Calibrating = 2,
	/// The calibration is complete and tracking can commence.
	Calibrated = 3
}

impl From<CalibrationState> for OscType {
	fn from(value: CalibrationState) -> Self {
		OscType::Int(value as i32)
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum CalibrationMode {
	Normal = 0,
	MixedRealityHand = 1,
	MixedRealityFloor = 2
}

impl From<CalibrationMode> for OscType {
	fn from(value: CalibrationMode) -> Self {
		OscType::Int(value as i32)
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

/// Quality of tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum TrackingState {
	/// Tracking is in poor condition (could be due to hitting the edge of the camera's view, or poor lighting)
	Poor = 0,
	/// Tracking is in good condition.
	Good = 1
}

impl From<TrackingState> for OscType {
	fn from(value: TrackingState) -> Self {
		OscType::Int(value as i32)
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
