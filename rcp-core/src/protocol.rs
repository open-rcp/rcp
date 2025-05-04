use crate::{Error, Frame, Result};
use bytes::BytesMut;
use std::fmt;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Connection state for the protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Initial connection state
    Connected,

    /// Authentication is in progress
    Authenticating,

    /// Successfully authenticated
    Authenticated,

    /// Connection is closing
    Closing,

    /// Connection is closed
    Closed,
}

/// Protocol handler for RCP connections
pub struct Protocol<T> {
    /// The underlying stream
    stream: T,

    /// Read buffer
    read_buffer: BytesMut,

    /// Connection state
    state: ConnectionState,
}

// Implement Debug trait for Protocol
impl<T> fmt::Debug for Protocol<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Protocol")
            .field("read_buffer_size", &self.read_buffer.len())
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl<T> Protocol<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Create a new protocol handler with the given stream
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            read_buffer: BytesMut::with_capacity(8192),
            state: ConnectionState::Connected,
        }
    }

    /// Get the current connection state
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Set the connection state
    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    /// Read the next frame from the stream
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // Try to parse a frame from the buffer
            if let Some(frame) = Frame::parse(&mut self.read_buffer)? {
                return Ok(Some(frame));
            }

            // Need more data, read from the stream
            let bytes_read = self.stream.read_buf(&mut self.read_buffer).await?;
            if bytes_read == 0 {
                if self.read_buffer.is_empty() {
                    // Clean EOF
                    return Ok(None);
                } else {
                    // Unexpected EOF in the middle of a frame
                    return Err(Error::ConnectionClosed);
                }
            }
        }
    }

    /// Write a frame to the stream
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        let bytes = frame.serialize();
        self.stream.write_all(&bytes).await?;
        Ok(())
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.set_state(ConnectionState::Closing);
        self.stream.shutdown().await?;
        self.set_state(ConnectionState::Closed);
        Ok(())
    }

    /// Take the underlying stream, consuming the protocol handler
    pub fn into_inner(self) -> T {
        self.stream
    }
}

/// A framed protocol connection with buffered frames
#[allow(dead_code)]
pub struct FramedProtocol<T> {
    /// The underlying protocol handler
    protocol: Protocol<T>,

    /// Queue of frames to be sent
    send_queue: Vec<Frame>,
}

#[allow(dead_code)]
impl<T> FramedProtocol<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Create a new framed protocol handler
    pub fn new(stream: T) -> Self {
        Self {
            protocol: Protocol::new(stream),
            send_queue: Vec::new(),
        }
    }

    /// Get the current connection state
    pub fn state(&self) -> ConnectionState {
        self.protocol.state()
    }

    /// Set the connection state
    pub fn set_state(&mut self, state: ConnectionState) {
        self.protocol.set_state(state);
    }

    /// Read the next frame from the stream
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        self.protocol.read_frame().await
    }

    /// Queue a frame to be sent
    pub fn queue_frame(&mut self, frame: Frame) {
        self.send_queue.push(frame);
    }

    /// Send all queued frames
    pub async fn flush(&mut self) -> Result<()> {
        for frame in self.send_queue.drain(..) {
            self.protocol.write_frame(&frame).await?;
        }
        Ok(())
    }

    /// Write a frame to the stream immediately
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        self.protocol.write_frame(frame).await
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.protocol.close().await
    }

    /// Take the underlying stream, consuming the protocol handler
    pub fn into_inner(self) -> T {
        self.protocol.into_inner()
    }
}

/// Thread-safe shared protocol for use across tasks
#[allow(dead_code)]
pub type SharedProtocol<T> = Arc<tokio::sync::Mutex<Protocol<T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    async fn test_protocol_read_write() {
        let (client, server) = duplex(1024);

        let mut client_proto = Protocol::new(client);
        let mut server_proto = Protocol::new(server);

        // Create a test frame
        let test_frame = Frame::new(0x01, b"Hello, RCP!".to_vec());

        // Client sends the frame
        tokio::spawn(async move {
            client_proto.write_frame(&test_frame).await.unwrap();
        });

        // Server reads the frame
        let received = server_proto.read_frame().await.unwrap().unwrap();

        assert_eq!(received.command_id(), 0x01);
        assert_eq!(received.payload(), b"Hello, RCP!");
    }

    #[tokio::test]
    async fn test_framed_protocol_queue_flush() {
        let (client, server) = duplex(1024);

        let mut client_proto = FramedProtocol::new(client);
        let mut server_proto = FramedProtocol::new(server);

        // Queue multiple frames
        client_proto.queue_frame(Frame::new(0x01, b"Frame1".to_vec()));
        client_proto.queue_frame(Frame::new(0x02, b"Frame2".to_vec()));
        client_proto.queue_frame(Frame::new(0x03, b"Frame3".to_vec()));

        // Client flushes all frames
        tokio::spawn(async move {
            client_proto.flush().await.unwrap();
        });

        // Server reads all frames
        let frame1 = server_proto.read_frame().await.unwrap().unwrap();
        let frame2 = server_proto.read_frame().await.unwrap().unwrap();
        let frame3 = server_proto.read_frame().await.unwrap().unwrap();

        assert_eq!(frame1.command_id(), 0x01);
        assert_eq!(frame1.payload(), b"Frame1");
        assert_eq!(frame2.command_id(), 0x02);
        assert_eq!(frame2.payload(), b"Frame2");
        assert_eq!(frame3.command_id(), 0x03);
        assert_eq!(frame3.payload(), b"Frame3");
    }
}
