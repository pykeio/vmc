use std::{
	fmt,
	future::Future,
	io,
	net::SocketAddr,
	pin::Pin,
	sync::Arc,
	task::{Context, Poll}
};

use tokio::net::UdpSocket;
use tokio_stream::Stream;

pub(crate) type RecvFuture = Pin<Box<dyn Future<Output = io::Result<(Vec<u8>, usize, SocketAddr)>> + Send + Sync>>;

pub(crate) struct UDPSocketStream {
	pub(crate) socket: Arc<UdpSocket>,
	future: Option<RecvFuture>,
	buf: Option<Vec<u8>>
}

unsafe impl Send for UDPSocketStream {}

impl Clone for UDPSocketStream {
	fn clone(&self) -> Self {
		Self::from_arc(self.socket.clone())
	}
}

impl fmt::Debug for UDPSocketStream {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("UdpSocketStream").field("socket", &*self.socket).finish()
	}
}

impl UDPSocketStream {
	pub fn new(socket: UdpSocket) -> Self {
		let socket = Arc::new(socket);
		Self::from_arc(socket)
	}

	pub fn from_arc(socket: Arc<UdpSocket>) -> Self {
		let buf = vec![0u8; 1024 * 64];
		Self { socket, future: None, buf: Some(buf) }
	}

	pub fn get_ref(&self) -> &UdpSocket {
		&self.socket
	}

	pub fn clone_inner(&self) -> Arc<UdpSocket> {
		Arc::clone(&self.socket)
	}
}

impl Stream for UDPSocketStream {
	type Item = io::Result<(Vec<u8>, SocketAddr)>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		loop {
			if self.future.is_none() {
				let buf = self.buf.take().unwrap();
				let future = recv_next(Arc::clone(&self.socket), buf);
				self.future = Some(Box::pin(future));
			}

			if let Some(f) = &mut self.future {
				let res = match f.as_mut().poll(cx) {
					Poll::Ready(t) => t,
					Poll::Pending => return Poll::Pending
				};
				self.future = None;
				return match res {
					Err(e) => Poll::Ready(Some(Err(e))),
					Ok((buf, n, addr)) => {
						let res_buf = buf[..n].to_vec();
						self.buf = Some(buf);
						Poll::Ready(Some(Ok((res_buf, addr))))
					}
				};
			}
		}
	}
}

async fn recv_next(socket: Arc<UdpSocket>, mut buf: Vec<u8>) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
	let res = socket.recv_from(&mut buf).await;
	match res {
		Err(e) => Err(e),
		Ok((n, addr)) => Ok((buf, n, addr))
	}
}
