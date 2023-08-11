use std::collections::HashMap;

use tokio_stream::StreamExt;
use vmc::{VMCMessage, VMCModelState, VMCResult};

#[tokio::main]
async fn main() -> VMCResult<()> {
	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;
	let mut blendshapes = HashMap::new();
	while let Some(packet) = socket.next().await {
		let (packet, _) = packet?;
		for message in vmc::parse(packet)? {
			match message {
				VMCMessage::BoneTransform(transform) => {
					println!("\tTransform bone: {} (pos {:?}; rot {:?})", transform.bone, transform.position, transform.rotation)
				}
				VMCMessage::DeviceTransform(transform) => {
					println!("\tTransform device ({:?}): {} (pos {:?}; rot {:?})", transform.device, transform.joint, transform.position, transform.rotation)
				}
				VMCMessage::RootTransform(transform) => {
					println!("\tTransform root: (pos {:?}; rot {:?})", transform.position, transform.rotation)
				}
				VMCMessage::State(t) => match t.model_state {
					VMCModelState::Loaded => println!("\tModel is loaded."),
					VMCModelState::NotLoaded => println!("\tModel is not yet loaded.")
				},
				VMCMessage::BlendShape(blend) => {
					blendshapes.insert(blend.key, blend.value);
				}
				VMCMessage::ApplyBlendShapes => {
					if !blendshapes.is_empty() {
						println!(
							"\tBlend shape: {}",
							blendshapes
								.iter()
								.filter(|b| b.1 > &0.)
								.map(|b| format!("{} x{:.02}", b.0, b.1))
								.collect::<Vec<_>>()
								.join(", ")
						);
						blendshapes.clear();
					}
				}
				VMCMessage::Time(t) => println!("Render all (time: {})", t.0)
			}
		}
	}

	Ok(())
}
