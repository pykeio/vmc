use std::time::Instant;

use glam::EulerRot;
use vmc::{
	Quat, VMCApplyBlendShapes, VMCBlendShape, VMCBoneTransform, VMCModelState, VMCResult, VMCStandardVRM0Bone, VMCStandardVRMBlendShape, VMCState, VMCTime,
	Vec3
};

#[tokio::main]
async fn main() -> VMCResult<()> {
	let start = Instant::now();
	let socket = vmc::performer!("127.0.0.1:39539").await?;
	loop {
		socket
			.send(VMCBlendShape::new(VMCStandardVRMBlendShape::A, start.elapsed().as_secs_f32().sin() / 2. + 0.5))
			.await?;
		socket
			.send(VMCBlendShape::new(VMCStandardVRMBlendShape::Fun, (start.elapsed().as_secs_f32().sin() / 2. + 0.5) * 0.6))
			.await?;
		socket
			.send(VMCBoneTransform::new(
				VMCStandardVRM0Bone::LeftEye,
				Vec3::new(-0.016136881, 0.061875343, 0.02154272),
				Quat::from_euler(EulerRot::XYZ, (start.elapsed().as_secs_f32().cos()) * 0.05, (start.elapsed().as_secs_f32().sin()) * 0.05, 0.)
			))
			.await?;
		socket
			.send(VMCBoneTransform::new(
				VMCStandardVRM0Bone::RightEye,
				Vec3::new(0.016136864, 0.061875224, 0.02154272),
				Quat::from_euler(EulerRot::XYZ, (start.elapsed().as_secs_f32().cos()) * 0.05, (start.elapsed().as_secs_f32().sin()) * 0.05, 0.)
			))
			.await?;
		socket.send(VMCApplyBlendShapes).await?;
		socket.send(VMCState::new(VMCModelState::Loaded)).await?;
		socket.send(VMCTime::elapsed()).await?;
	}
}
