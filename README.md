# `vmc`
An asynchronous implementation of the [Virtual Motion Capture Protocol](https://protocol.vmc.info/) in Rust.

While this crate is intended specifically for Virtual Motion Capture, it can also be used as an implementation of the [Open Sound Control](https://opensoundcontrol.stanford.edu/) protocol which VMC is based on.

## Examples
See [`examples/`](https://github.com/vitri-ent/vmc/tree/main/examples/) for more detailed examples.

### Performer
```rs
use vmc::{
	VMCApplyBlendShapes, VMCBlendShape, VMCModelState, VMCResult, VMCStandardVRMBlendShape, VMCState, VMCTime
};

#[tokio::main]
async fn main() -> VMCResult<()> {
	let socket = vmc::performer!("127.0.0.1:39539").await?;
	loop {
		socket
			.send(VMCBlendShape::new(VMCStandardVRMBlendShape::Joy, 1.0))
			.await?;
		socket.send(VMCApplyBlendShapes).await?;
		socket.send(VMCState::new(VMCModelState::Loaded)).await?;
		socket.send(VMCTime::elapsed()).await?;
	}
}
```

### Marionette
```rs
use tokio_stream::StreamExt;
use vmc::{VMCMessage, VMCResult};

#[tokio::main]
async fn main() -> VMCResult<()> {
	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;
	while let Some(packet) = socket.next().await {
		let (packet, _) = packet?;
		for message in vmc::parse(packet)? {
			match message {
				VMCMessage::BoneTransform(transform) => {
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
❤️ This package is based on [`rosc`](https://github.com/klingtnet/rosc/blob/master/Cargo.toml) by Andreas Linz and [`async-osc`](https://github.com/Frando/async-osc) by Franz Heinzmann. Licensed under MIT License or Apache-2.0.
