use bytes::BytesMut;
use rcp_core::*;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};

/// Test implementation of AsyncRead/AsyncWrite for testing protocol
struct TestConnection {
    read_data: BytesMut,
    write_data: BytesMut,
}

impl TestConnection {
    fn new() -> Self {
        Self {
            read_data: BytesMut::new(),
            write_data: BytesMut::new(),
        }
    }

    fn with_data(data: &[u8]) -> Self {
        let mut conn = Self::new();
        conn.read_data.extend_from_slice(data);
        conn
    }

    fn written_data(&self) -> &[u8] {
        &self.write_data
    }
}

impl AsyncRead for TestConnection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.read_data.is_empty() {
            return Poll::Ready(Ok(()));
        }

        let to_read = std::cmp::min(buf.remaining(), self.read_data.len());
        let data = self.read_data.split_to(to_read);
        buf.put_slice(&data);

        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for TestConnection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.write_data.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[tokio::test]
async fn test_protocol_state_transitions() {
    // Test protocol state transitions through authentication flow
    let conn = TestConnection::new();
    let mut proto = Protocol::new(conn);

    // Check initial state
    assert_eq!(proto.state(), ConnectionState::Connected);

    // Set state manually for testing
    proto.set_state(ConnectionState::Authenticating);
    assert_eq!(proto.state(), ConnectionState::Authenticating);

    // Set state to authenticated
    proto.set_state(ConnectionState::Authenticated);
    assert_eq!(proto.state(), ConnectionState::Authenticated);
}

#[tokio::test]
async fn test_protocol_send_receive() {
    // Test sending and receiving frames through the protocol
    let conn = TestConnection::new();
    let mut proto = Protocol::new(conn);

    // Create a simple frame with command ID for LaunchApp
    let command_id = CommandId::LaunchApp as u8;

    // Create a LaunchApp command with proper data structure
    let launch_app = LaunchAppCommand {
        flags: 0,
        application_path: "test-app".to_string(),
        args: Some("--test".to_string()),
    };

    // Serialize the command payload
    let payload = serde_json::to_vec(&launch_app).unwrap_or_default();
    let frame = Frame::new(command_id, payload);

    // Send the frame
    proto
        .write_frame(&frame)
        .await
        .expect("Failed to write frame");

    // Extract connection to check what was written
    let inner_conn = proto.into_inner();
    let written_data = inner_conn.written_data();

    // Verify data was written
    assert!(!written_data.is_empty());
}

#[tokio::test]
async fn test_protocol_bidirectional() {
    // Test bidirectional communication with framed protocol

    // Create a frame to send
    let command_id = CommandId::Ping as u8;
    let send_frame = Frame::new(command_id, b"ping".to_vec());
    let serialized = send_frame.serialize();

    // Initialize connection with the serialized data
    let conn = TestConnection::with_data(&serialized);
    let mut proto = Protocol::new(conn);

    // Read the frame back from the connection
    let received_frame = proto
        .read_frame()
        .await
        .expect("Read error")
        .expect("No frame");

    // Verify the frame matches what was sent
    assert_eq!(received_frame.command_id(), command_id);
    assert_eq!(received_frame.payload(), b"ping");

    // Create a response frame and write it
    let response_frame = Frame::new(CommandId::Ack as u8, b"pong".to_vec());
    proto
        .write_frame(&response_frame)
        .await
        .expect("Failed to write frame");

    // Extract the connection to check the written data
    let inner_conn = proto.into_inner();
    let written_data = inner_conn.written_data();

    // Verify response was written
    assert!(!written_data.is_empty());

    // We could parse the written data back into a frame for full verification
    let mut buffer = BytesMut::from(written_data);
    let parsed_response = Frame::parse(&mut buffer).unwrap().unwrap();
    assert_eq!(parsed_response.command_id(), CommandId::Ack as u8);
    assert_eq!(parsed_response.payload(), b"pong");
}
