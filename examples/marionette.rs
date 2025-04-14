use std::collections::HashMap;

use futures_util::StreamExt;
use vmc::{Message, ModelState};

#[tokio::main]
async fn main() -> vmc::Result<()> {
	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;
	let mut blendshapes = HashMap::new();
	while let Some(packet) = socket.next().await {
		let (packet, _) = packet?;
		for message in vmc::parse(packet)? {
			match message {
				Message::BoneTransform(transform) => {
					println!("\tTransform bone: {} (pos {:?}; rot {:?})", transform.bone, transform.position, transform.rotation)
				}
				Message::DeviceTransform(transform) => {
					println!("\tTransform device ({:?}): {} (pos {:?}; rot {:?})", transform.device, transform.joint, transform.position, transform.rotation)
				}
				Message::RootTransform(transform) => {
					println!("\tTransform root: (pos {:?}; rot {:?})", transform.position, transform.rotation)
				}
				Message::State(t) => match t.model_state {
					ModelState::Loaded => println!("\tModel is loaded."),
					ModelState::NotLoaded => println!("\tModel is not yet loaded.")
				},
				Message::BlendShape(blend) => {
					blendshapes.insert(blend.key, blend.value);
				}
				Message::ApplyBlendShapes => {
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
				Message::Time(t) => println!("Render all (time: {})", t.0)
			}
		}
	}

	Ok(())
}
