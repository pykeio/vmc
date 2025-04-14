# `vmc`
An asynchronous implementation of the [Virtual Motion Capture Protocol](https://protocol.vmc.info/) in Rust.

## Examples
See [`examples/`](https://github.com/vitri-ent/vmc/tree/main/examples/) for more detailed examples.

### Performer
```rs
use vmc::{ApplyBlendShapes, BlendShape, ModelState, StandardVRMBlendShape, State, Time};

#[tokio::main]
async fn main() -> vmc::Result<()> {
	let socket = vmc::performer!("127.0.0.1:39539").await?;
	loop {
		socket.send(BlendShape::new(StandardVRMBlendShape::Joy, 1.0)).await?;
		socket.send(ApplyBlendShapes).await?;
		socket.send(State::new(ModelState::Loaded)).await?;
		socket.send(Time::elapsed()).await?;
	}
}
```

### Marionette
```rs
use tokio_stream::StreamExt;
use vmc::Message;

#[tokio::main]
async fn main() -> vmc::Result<()> {
	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;
	while let Some(packet) = socket.next().await {
		let (packet, _) = packet?;
		for message in vmc::parse(packet)? {
			match message {
				Message::BoneTransform(transform) => {
					println!("\tTransform bone: {} (pos {:?}; rot {:?})", transform.bone, transform.position, transform.rotation)
				}
				_ => {}
			}
		}
	}

	Ok(())
}
```

## License
❤️ This crate is based on [`rosc`](https://github.com/klingtnet/rosc) by Andreas Linz and [`async-osc`](https://github.com/Frando/async-osc) by Franz Heinzmann. Licensed under MIT License or Apache-2.0.
