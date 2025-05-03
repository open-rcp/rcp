use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Command IDs for the RCP protocol.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandId {
    LaunchApp = 0x01,
    SendInput = 0x02,
    StreamFrame = 0x03,
    ResizeWindow = 0x04,
    ClipboardData = 0x05,
    FileTransfer = 0x06,
    AudioData = 0x07,
    DisplayInfo = 0x08,
    CursorPosition = 0x09,
    PermissionRequest = 0x0A,
    ServiceSubscribe = 0x0B,
    VideoQuality = 0x0C,
    PrivacyMode = 0x0D,
    WindowFocus = 0x0E,
    Ping = 0xF0,
    Error = 0xF1,
    Auth = 0xFE,
    Heartbeat = 0xFF,
}

impl TryFrom<u8> for CommandId {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(CommandId::LaunchApp),
            0x02 => Ok(CommandId::SendInput),
            0x03 => Ok(CommandId::StreamFrame),
            0x04 => Ok(CommandId::ResizeWindow),
            0x05 => Ok(CommandId::ClipboardData),
            0x06 => Ok(CommandId::FileTransfer),
            0x07 => Ok(CommandId::AudioData),
            0x08 => Ok(CommandId::DisplayInfo),
            0x09 => Ok(CommandId::CursorPosition),
            0x0A => Ok(CommandId::PermissionRequest),
            0x0B => Ok(CommandId::ServiceSubscribe),
            0x0C => Ok(CommandId::VideoQuality),
            0x0D => Ok(CommandId::PrivacyMode),
            0x0E => Ok(CommandId::WindowFocus),
            0xF0 => Ok(CommandId::Ping),
            0xF1 => Ok(CommandId::Error),
            0xFE => Ok(CommandId::Auth),
            0xFF => Ok(CommandId::Heartbeat),
            _ => Err(Error::InvalidCommand(value)),
        }
    }
}

/// Base trait for all RCP commands
pub trait Command {
    /// Returns the command ID for this command
    fn command_id(&self) -> CommandId;
    
    /// Serializes the command to bytes
    fn serialize(&self) -> Result<Vec<u8>>;
    
    /// Parses a command from bytes
    fn parse(payload: &[u8]) -> Result<Self> where Self: Sized;
}

/// LaunchApp command payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchAppCommand {
    /// Launch flags
    pub flags: u32,
    
    /// Application path to launch
    pub application_path: String,
    
    /// Command line arguments, if any
    pub args: Option<String>,
}

/// Mouse input event types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseEventType {
    Move = 0x01,
    LeftDown = 0x02,
    LeftUp = 0x03,
    RightDown = 0x04,
    RightUp = 0x05,
    MiddleDown = 0x06,
    MiddleUp = 0x07,
    Wheel = 0x08,
}

/// Mouse input event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    /// Type of mouse event
    pub event_type: MouseEventType,
    
    /// X position
    pub x: i32,
    
    /// Y position
    pub y: i32,
    
    /// Wheel delta (for MouseEventType::Wheel)
    pub wheel_delta: i16,
}

/// Keyboard key codes (platform independent)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyCode {
    Unknown = 0,
    
    // Letters
    A = 0x0041,
    B = 0x0042,
    C = 0x0043,
    // ... other letters ...
    Z = 0x005A,
    
    // Numbers
    N0 = 0x0030,
    N1 = 0x0031,
    N2 = 0x0032,
    N3 = 0x0033,
    N4 = 0x0034,
    N5 = 0x0035,
    N6 = 0x0036,
    N7 = 0x0037,
    N8 = 0x0038,
    N9 = 0x0039,
    
    // Function keys
    F1 = 0x0070,
    F2 = 0x0071,
    F3 = 0x0072,
    F4 = 0x0073,
    F5 = 0x0074,
    F6 = 0x0075,
    F7 = 0x0076,
    F8 = 0x0077,
    F9 = 0x0078,
    F10 = 0x0079,
    F11 = 0x007A,
    F12 = 0x007B,
    
    // Control keys
    Backspace = 0x0008,
    Tab = 0x0009,
    Enter = 0x000D,
    Shift = 0x0010,
    Control = 0x0011,
    Alt = 0x0012,
    Pause = 0x0013,
    CapsLock = 0x0014,
    Escape = 0x001B,
    Space = 0x0020,
    PageUp = 0x0021,
    PageDown = 0x0022,
    End = 0x0023,
    Home = 0x0024,
    Left = 0x0025,
    Up = 0x0026,
    Right = 0x0027,
    Down = 0x0028,
    PrintScreen = 0x002C,
    Insert = 0x002D,
    Delete = 0x002E,
    
    // Numpad
    NumLock = 0x0090,
    NumPad0 = 0x0060,
    NumPad1 = 0x0061,
    NumPad2 = 0x0062,
    NumPad3 = 0x0063,
    NumPad4 = 0x0064,
    NumPad5 = 0x0065,
    NumPad6 = 0x0066,
    NumPad7 = 0x0067,
    NumPad8 = 0x0068,
    NumPad9 = 0x0069,
    NumPadMultiply = 0x006A,
    NumPadAdd = 0x006B,
    NumPadSubtract = 0x006D,
    NumPadDecimal = 0x006E,
    NumPadDivide = 0x006F,
}

/// Keyboard state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    Released = 0,
    Pressed = 1,
}

/// Keyboard modifier flags
pub type ModifierFlags = u8;

/// Keyboard modifier keys bit flags
pub const MODIFIER_SHIFT: ModifierFlags = 0x01;
pub const MODIFIER_CONTROL: ModifierFlags = 0x02;
pub const MODIFIER_ALT: ModifierFlags = 0x04;
pub const MODIFIER_META: ModifierFlags = 0x08;
pub const MODIFIER_CAPS_LOCK: ModifierFlags = 0x10;
pub const MODIFIER_NUM_LOCK: ModifierFlags = 0x20;

/// Keyboard input event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    /// Key code
    pub key_code: KeyCode,
    
    /// Key state
    pub state: KeyState,
    
    /// Active modifiers
    pub modifiers: ModifierFlags,
}

/// Input command payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputCommand {
    Mouse(MouseEvent),
    Keyboard(KeyEvent),
}

/// Streaming frame format
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameFormat {
    Raw = 0,
    Jpeg = 1,
    Png = 2,
    Vp8 = 3,
    Vp9 = 4,
    H264 = 5,
    H265 = 6,
}

/// Streaming frame flags
pub type FrameFlags = u16;

/// Frame is a keyframe
pub const FRAME_FLAG_KEYFRAME: FrameFlags = 0x0001;
/// Frame contains cursor
pub const FRAME_FLAG_CURSOR: FrameFlags = 0x0002;
/// Frame is a partial update
pub const FRAME_FLAG_PARTIAL: FrameFlags = 0x0004;

/// Video frame payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamFrameCommand {
    /// Frame width
    pub width: u32,
    
    /// Frame height
    pub height: u32,
    
    /// Display ID
    pub display_id: u32,
    
    /// Frame format
    pub format: FrameFormat,
    
    /// Frame flags
    pub flags: FrameFlags,
    
    /// Video quality (0-100)
    pub quality: u16,
    
    /// Encoded frame data
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

// Additional command implementations can be added here