use std::time::Instant;

use glam::EulerRot;
use vmc::{ApplyBlendShapes, BlendShape, BoneTransform, ModelState, StandardVRM0Bone, StandardVRMBlendShape, State, Time, Vec3};

#[tokio::main]
async fn main() -> vmc::Result<()> {
	let start = Instant::now();
	let socket = vmc::performer!("127.0.0.1:39539").await?;
	loop {
		socket
			.send(BlendShape::new(StandardVRMBlendShape::A, start.elapsed().as_secs_f32().sin() / 2. + 0.5))
			.await?;
		socket
			.send(BlendShape::new(StandardVRMBlendShape::Fun, (start.elapsed().as_secs_f32().sin() / 2. + 0.5) * 0.6))
			.await?;
		socket
			.send(BoneTransform::new(
				StandardVRM0Bone::LeftEye,
				Vec3::new(-0.016136881, 0.061875343, 0.02154272),
				glam::Quat::from_euler(EulerRot::XYZ, (start.elapsed().as_secs_f32().cos()) * 0.05, (start.elapsed().as_secs_f32().sin()) * 0.05, 0.)
					.to_array()
			))
			.await?;
		socket
			.send(BoneTransform::new(
				StandardVRM0Bone::RightEye,
				Vec3::new(0.016136864, 0.061875224, 0.02154272),
				glam::Quat::from_euler(EulerRot::XYZ, (start.elapsed().as_secs_f32().cos()) * 0.05, (start.elapsed().as_secs_f32().sin()) * 0.05, 0.)
					.to_array()
			))
			.await?;
		socket.send(ApplyBlendShapes).await?;
		socket.send(State::new(ModelState::Loaded)).await?;
		socket.send(Time::elapsed()).await?;
	}
}
