use std::sync::{
	Arc, RwLock,
	atomic::{AtomicBool, Ordering}
};

use console::Term;
use futures_util::StreamExt;
use serde::Serialize;
use vmc::{VMCMessage, VMCResult};

#[derive(Default, Serialize)]
struct MessageBundle {
	time_delta: f32,
	messages: Vec<VMCMessage>
}

#[tokio::main]
async fn main() -> VMCResult<()> {
	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;

	tokio::spawn(async move {
		tokio::signal::ctrl_c().await.unwrap();
		std::process::exit(0);
	});

	let packet_buffer = Arc::new(RwLock::new(Vec::new()));
	let mut current_packet = MessageBundle::default();
	let active = Arc::new(AtomicBool::new(false));

	let _packet_buffer = Arc::clone(&packet_buffer);
	let _active = Arc::clone(&active);
	std::thread::spawn(move || {
		let term = Term::stdout();
		while term.read_char().is_ok() {
			let active = _active.load(Ordering::Relaxed);
			if active {
				let mut packet_buffer = _packet_buffer.write().unwrap();
				let buf = &packet_buffer[1..];
				std::fs::write("out.vmc", rmp_serde::to_vec(buf).unwrap()).unwrap();
				packet_buffer.clear();
				println!("Stopped");
			} else {
				println!("Started");
			}
			_active.store(!active, Ordering::Relaxed);
		}
	});

	while let Some(packet) = socket.next().await {
		let (packet, _) = packet?;
		for message in vmc::parse(packet)? {
			if active.load(Ordering::Relaxed) {
				match message {
					VMCMessage::Time(t) => {
						{
							let mut packet_buffer = packet_buffer.write().unwrap();
							packet_buffer.push(current_packet);
						}
						current_packet = MessageBundle::default();

						current_packet.time_delta = t.0;
					}
					message => current_packet.messages.push(message)
				}
			}
		}
	}

	Ok(())
}
