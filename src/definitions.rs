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
