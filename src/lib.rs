//! # `vmc`
//! An asynchronous implementation of the [Virtual Motion Capture Protocol](https://protocol.vmc.info/) in Rust.
//!
//! While this crate is intended specifically for Virtual Motion Capture, it can also be used as an implementation of
//! the [Open Sound Control](https://opensoundcontrol.stanford.edu/) protocol which VMC is based on; see [`crate::osc`].
//!
//! ## Examples
//! See [`examples/`](https://github.com/vitri-ent/vmc/tree/main/examples/) for more detailed examples.
//!
//! ### Performer
//! ```no_run
//! use vmc::{
//! 	VMCApplyBlendShapes, VMCBlendShape, VMCModelState, VMCResult, VMCStandardVRMBlendShape, VMCState, VMCTime
//! };
//!
//! #[tokio::main]
//! async fn main() -> VMCResult<()> {
//! 	let socket = vmc::performer!("127.0.0.1:39539").await?;
//! 	loop {
//! 		socket.send(VMCBlendShape::new(VMCStandardVRMBlendShape::Joy, 1.0)).await?;
//! 		socket.send(VMCApplyBlendShapes).await?;
//! 		socket.send(VMCState::new(VMCModelState::Loaded)).await?;
//! 		socket.send(VMCTime::elapsed()).await?;
//! 	}
//! }
//! ```
//!
//! ### Marionette
//! ```no_run
//! use tokio_stream::StreamExt;
//! use vmc::{VMCMessage, VMCResult};
//!
//! #[tokio::main]
//! async fn main() -> VMCResult<()> {
//! 	let mut socket = vmc::marionette!("127.0.0.1:39539").await?;
//! 	while let Some(packet) = socket.next().await {
//! 		let (packet, _) = packet?;
//! 		for message in vmc::parse(packet)? {
//! 			match message {
//! 				VMCMessage::BoneTransform(transform) => {
//! 					println!(
//! 						"\tTransform bone: {} (pos {:?}; rot {:?})",
//! 						transform.bone, transform.position, transform.rotation
//! 					)
//! 				}
//! 				_ => {}
//! 			}
//! 		}
//! 	}
//!
//! 	Ok(())
//! }
//! ```
//!
//! ## License
//! ❤️ This package is based on [`rosc`](https://github.com/klingtnet/rosc/blob/master/Cargo.toml) by Andreas Linz and
//! [`async-osc`](https://github.com/Frando/async-osc) by Franz Heinzmann. Licensed under MIT License or Apache-2.0.

#![allow(clippy::tabs_in_doc_comments)]

use std::{
	io,
	net::SocketAddr,
	pin::Pin,
	sync::Arc,
	task::{Context, Poll}
};

use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio_stream::Stream;

mod error;
pub mod message;
pub mod osc;
mod udp;

pub use glam::{EulerRot, Quat, Vec3, Vec3A};

use self::udp::UDPSocketStream;
pub use self::{
	error::{VMCError, VMCResult},
	message::{
		ApplyBlendShapes as VMCApplyBlendShapes, BlendShape as VMCBlendShape, BoneTransform as VMCBoneTransform, CalibrationMode as VMCCalibrationMode,
		CalibrationState as VMCCalibrationState, DeviceTransform as VMCDeviceTransform, DeviceType as VMCDeviceType, ModelState as VMCModelState,
		RootTransform as VMCRootTransform, StandardVRM0Bone as VMCStandardVRM0Bone, StandardVRMBlendShape as VMCStandardVRMBlendShape, State as VMCState,
		Time as VMCTime, TrackingState as VMCTrackingState, VMCMessage, parse
	},
	osc::{IntoOSCArgs, IntoOSCMessage, IntoOSCPacket, OSCPacket, OSCType}
};

/// A UDP socket to send and receive VMC messages.
#[derive(Debug)]
pub struct VMCSocket {
	socket: UDPSocketStream
}

impl VMCSocket {
	/// Creates a new OSC socket from a [`tokio::net::UdpSocket`].
	pub fn new(socket: UdpSocket) -> Self {
		let socket = UDPSocketStream::new(socket);
		Self { socket }
	}

	/// Creates an VMC socket from the given address.
	///
	/// Binding with a port number of 0 will request that the OS assigns a port to this socket.
	/// The port allocated can be queried via [`local_addr`] method.
	///
	/// [`local_addr`]: #method.local_addr
	pub async fn bind<A: ToSocketAddrs>(addr: A) -> VMCResult<Self> {
		let socket = UdpSocket::bind(addr).await?;
		Ok(Self::new(socket))
	}

	/// Connects the UDP socket to a remote address.
	///
	/// When connected, only messages from this address will be received and the [`send`] method
	/// will use the specified address for sending.
	///
	/// [`send`]: #method.send
	///
	/// # Examples
	///
	/// ```no_run
	/// # fn main() -> vmc::VMCResult<()> { tokio_test::block_on(async {
	/// use vmc::VMCSocket;
	///
	/// let socket = VMCSocket::bind("127.0.0.1:0").await?;
	/// socket.connect("127.0.0.1:8080").await?;
	/// # Ok(()) }) }
	/// ```
	pub async fn connect<A: ToSocketAddrs>(&self, addrs: A) -> VMCResult<()> {
		self.socket().connect(addrs).await?;
		Ok(())
	}

	/// Sends an OSC packet on the socket to the given address.
	///
	/// # Examples
	///
	/// ```no_run
	/// # fn main() -> vmc::VMCResult<()> { tokio_test::block_on(async {
	/// use vmc::{VMCBlendShape, VMCSocket, VMCStandardVRMBlendShape};
	///
	/// let socket = VMCSocket::bind("127.0.0.1:0").await?;
	/// let addr = "127.0.0.1:39539";
	/// let message = VMCBlendShape::new(VMCStandardVRMBlendShape::Joy, 1.0);
	/// socket.send_to(message, &addr).await?;
	/// # Ok(()) }) }
	/// ```
	pub async fn send_to<A: ToSocketAddrs, P: IntoOSCPacket>(&self, packet: P, addrs: A) -> VMCResult<()> {
		let buf = self::osc::encode(&packet.into_osc_packet())?;
		let n = self.socket().send_to(&buf[..], addrs).await?;
		check_len(&buf[..], n)
	}

	/// Sends a packet on the socket to the remote address to which it is connected.
	///
	/// The [`connect`] method will connect this socket to a remote address.
	/// This method will fail if the socket is not connected.
	///
	/// [`connect`]: #method.connect
	///
	/// # Examples
	///
	/// ```no_run
	/// # fn main() -> vmc::VMCResult<()> { tokio_test::block_on(async {
	/// use vmc::{VMCBlendShape, VMCSocket, VMCStandardVRMBlendShape};
	///
	/// let socket = VMCSocket::bind("127.0.0.1:2434").await?;
	/// socket.connect("127.0.0.1:39539").await?;
	/// socket.send(VMCBlendShape::new(VMCStandardVRMBlendShape::Joy, 1.0)).await?;
	/// #
	/// # Ok(()) }) }
	/// ```
	pub async fn send<P: IntoOSCPacket>(&self, packet: P) -> VMCResult<()> {
		let buf = self::osc::encode(&packet.into_osc_packet())?;
		let n = self.socket().send(&buf[..]).await?;
		check_len(&buf[..], n)
	}

	/// Create a standalone sender for this socket.
	///
	/// The sender can be moved to other threads or tasks.
	pub fn sender(&self) -> VMCSender {
		VMCSender::new(self.socket.clone_inner())
	}

	/// Get a reference to the underling [`UdpSocket`].
	pub fn socket(&self) -> &UdpSocket {
		self.socket.get_ref()
	}

	/// Returns the local address that this socket is bound to.
	///
	/// This can be useful, for example, when binding to port 0 to figure out which port was
	/// actually bound.
	pub fn local_addr(&self) -> VMCResult<SocketAddr> {
		let addr = self.socket().local_addr()?;
		Ok(addr)
	}
}

impl Stream for VMCSocket {
	type Item = VMCResult<(OSCPacket, SocketAddr)>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let packet = match Pin::new(&mut self.socket).poll_next(cx) {
			Poll::Ready(packet) => packet,
			Poll::Pending => return Poll::Pending
		};
		let message = packet.map(|packet| match packet {
			Err(err) => Err(err.into()),
			Ok((buf, peer_addr)) => self::osc::decode_udp(&buf[..]).map_err(|e| e.into()).map(|p| (p.1, peer_addr))
		});
		Poll::Ready(message)
	}
}

/// A sender to send messages over a VMC socket.
///
/// See [`VMCSocket::sender`].
#[derive(Clone, Debug)]
pub struct VMCSender {
	socket: Arc<UdpSocket>
}

impl VMCSender {
	fn new(socket: Arc<UdpSocket>) -> Self {
		Self { socket }
	}

	/// Sends a VMC packet on the socket to the given address.
	///
	/// See [`VMCSocket::send_to`].
	pub async fn send_to<A: ToSocketAddrs, P: IntoOSCPacket>(&self, packet: P, addrs: A) -> VMCResult<()> {
		let buf = self::osc::encode(&packet.into_osc_packet())?;
		let n = self.socket().send_to(&buf[..], addrs).await?;
		check_len(&buf[..], n)
	}

	/// Sends a VMC packet on the connected socket.
	///
	/// See [`VMCSocket::send`].
	pub async fn send<P: IntoOSCPacket>(&self, packet: P) -> VMCResult<()> {
		let buf = self::osc::encode(&packet.into_osc_packet())?;
		let n = self.socket().send(&buf[..]).await?;
		check_len(&buf[..], n)
	}

	/// Get a reference to the underling [`UdpSocket`].
	pub fn socket(&self) -> &UdpSocket {
		&self.socket
	}
}

/// Creates a new VMC Performer. Performers process tracking, motion, and IK, and send bone transforms and other
/// information to a [`marionette`].
///
/// The default usage of this usage will automatically select a free port to bind to, and send information to port 39539
/// (default VMC receiver port) on the local machine.
///
/// The behavior of both binding and sending can be customized:
/// ```no_run
/// # fn main() -> vmc::VMCResult<()> { tokio_test::block_on(async {
/// // default; binds to random port, sends to 127.0.0.1:39539
/// let performer = vmc::performer!().await?;
/// // customize bound port
/// let performer = vmc::performer!(bind_port = 39540).await?;
/// // customize bind address
/// let performer = vmc::performer!(bind = "192.168.1.182:39539").await?;
/// // customize send address & port
/// let performer = vmc::performer!("127.13.72.16:2434").await?;
/// // customize send address and bound port
/// let performer = vmc::performer!("127.13.72.16:2434", bind_port = 39540).await?;
/// # Ok(()) }) }
/// ```
#[macro_export]
macro_rules! performer {
	() => {
		$crate::_create_performer("127.0.0.1:0", "127.0.0.1:39539")
	};
	(bind = $bind:expr) => {
		$crate::_create_performer($bind, "127.0.0.1:39539")
	};
	(bind_port = $bind_port:expr) => {
		$crate::_create_performer(format!("127.0.0.1:{}", $bind_port), "127.0.0.1:39539")
	};
	($addr:expr) => {
		$crate::_create_performer("127.0.0.1:0", $addr)
	};
	($addr:expr, bind = $bind:expr) => {
		$crate::_create_performer($bind, $addr)
	};
	($addr:expr, bind_port = $bind_port:expr) => {
		$crate::_create_performer(format!("127.0.0.1:{}", $bind_port), $addr)
	};
}

#[doc(hidden)]
pub async fn _create_performer(bind: impl ToSocketAddrs, addr: impl ToSocketAddrs) -> VMCResult<VMCSocket> {
	let socket = VMCSocket::bind(bind).await?;
	socket.connect(addr).await?;
	Ok(socket)
}

/// Creates a new VMC Marionette. Marionettes receive motion data from a [`performer`] and render the avatar to a
/// screen.
///
/// The default usage of this usage will automatically bind to port 39539, the default VMC port.
///
/// The binding address can also be customized:
/// ```no_run
/// # fn main() -> vmc::VMCResult<()> { tokio_test::block_on(async {
/// // default; binds to 127.0.0.1:39539
/// let marionette = vmc::marionette!().await?;
/// // customize bind address/port
/// let marionette = vmc::marionette!("192.168.1.193:2434").await?;
/// # Ok(()) }) }
/// ```
#[macro_export]
macro_rules! marionette {
	() => {
		$crate::_create_marionette("127.0.0.1:39539")
	};
	($addr:expr) => {
		$crate::_create_marionette($addr)
	};
}

#[doc(hidden)]
pub async fn _create_marionette(addr: impl ToSocketAddrs) -> VMCResult<VMCSocket> {
	let socket = VMCSocket::bind(addr).await?;
	Ok(socket)
}

fn check_len(buf: &[u8], len: usize) -> VMCResult<()> {
	if len != buf.len() {
		Err(io::Error::new(io::ErrorKind::Interrupted, "UDP packet not fully sent").into())
	} else {
		Ok(())
	}
}
