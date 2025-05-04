use crate::error::Result;
use async_trait::async_trait;
use rcp_core::Frame;
use std::fmt::Debug;

/// Service trait for implementing different RCP services
#[async_trait]
pub trait Service: Debug + Send + Sync {
    /// Get the name of the service
    #[allow(dead_code)]
    fn name(&self) -> &str;

    /// Start the service
    async fn start(&mut self) -> Result<()>;

    /// Stop the service
    async fn stop(&mut self) -> Result<()>;

    /// Process an incoming frame from the client
    async fn process_frame(&mut self, frame: &Frame) -> Result<()>;

    /// Get a frame from the service (if available)
    async fn get_frame(&mut self) -> Result<Option<Frame>>;
}

/// Factory for creating service instances
pub mod services {
    use super::*;
    use crate::error::Error;
    use log::{error, info};
    use rcp_core::{CommandId, Frame, LaunchAppCommand};
    use serde_json;
    use std::process::Command;
    use std::time::Duration;
    use tokio::sync::mpsc::{self, Receiver, Sender};
    use uuid::Uuid;

    /// Display service for screen sharing
    #[derive(Debug)]
    pub struct DisplayService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Sender for outgoing frames
        frame_sender: Sender<Frame>,

        /// Receiver for outgoing frames
        frame_receiver: Receiver<Frame>,

        /// Whether the service is running
        running: bool,
    }

    impl Default for DisplayService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl DisplayService {
        /// Create a new display service
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel(100);

            Self {
                name: "display".to_string(),
                id: Uuid::new_v4(),
                frame_sender: tx,
                frame_receiver: rx,
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for DisplayService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service(
                    "Display service already running".to_string(),
                ));
            }

            self.running = true;

            // TODO: Implement actual screen capture
            // For now, just send a dummy frame periodically for testing

            let sender = self.frame_sender.clone();
            tokio::spawn(async move {
                loop {
                    // Create a dummy frame with some metadata
                    let dummy_data =
                        r#"{"width":1280,"height":720,"format":"jpeg"}"#.as_bytes().to_vec();
                    let frame = Frame::new(CommandId::DisplayInfo as u8, dummy_data);

                    if sender.send(frame).await.is_err() {
                        break;
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });

            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Display service not running".to_string()));
            }

            self.running = false;

            // The channel will be dropped when this object is dropped
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            // Display service mainly receives configuration frames from client
            match frame.command_id() {
                cmd if cmd == CommandId::VideoQuality as u8 => {
                    // TODO: Adjust quality based on frame content
                    log::info!("Received video quality settings");
                }
                cmd if cmd == CommandId::SubscribeDisplay as u8 => {
                    // TODO: Start streaming
                    log::info!("Received start stream request");
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Display service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            if !self.running {
                return Err(Error::Service("Display service not running".to_string()));
            }

            match self.frame_receiver.try_recv() {
                Ok(frame) => Ok(Some(frame)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(mpsc::error::TryRecvError::Disconnected) => Err(Error::Service(
                    "Display service channel disconnected".to_string(),
                )),
            }
        }
    }

    /// Input service for handling keyboard and mouse input
    #[derive(Debug)]
    pub struct InputService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Whether the service is running
        running: bool,
    }

    impl Default for InputService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InputService {
        /// Create a new input service
        pub fn new() -> Self {
            Self {
                name: "input".to_string(),
                id: Uuid::new_v4(),
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for InputService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service("Input service already running".to_string()));
            }

            self.running = true;
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Input service not running".to_string()));
            }

            self.running = false;
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Input service not running".to_string()));
            }

            match frame.command_id() {
                cmd if cmd == CommandId::SendInput as u8 => {
                    // TODO: Process input event
                    log::info!(
                        "Received input event, payload size: {}",
                        frame.payload().len()
                    );
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Input service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            // Input service doesn't generate frames
            Ok(None)
        }
    }

    /// Clipboard service for synchronizing clipboard content
    #[derive(Debug)]
    pub struct ClipboardService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Whether the service is running
        running: bool,
    }

    impl Default for ClipboardService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ClipboardService {
        /// Create a new clipboard service
        pub fn new() -> Self {
            Self {
                name: "clipboard".to_string(),
                id: Uuid::new_v4(),
                running: false,
            }
        }
    }

    #[async_trait]
    impl Service for ClipboardService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service(
                    "Clipboard service already running".to_string(),
                ));
            }

            self.running = true;
            // TODO: Setup clipboard monitoring
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Clipboard service not running".to_string()));
            }

            self.running = false;
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            if !self.running {
                return Err(Error::Service("Clipboard service not running".to_string()));
            }

            match frame.command_id() {
                cmd if cmd == CommandId::ClipboardData as u8 => {
                    // TODO: Process clipboard data from client
                    log::info!(
                        "Received clipboard data, size: {} bytes",
                        frame.payload().len()
                    );
                }
                _ => {
                    return Err(Error::Protocol(format!(
                        "Clipboard service: Unsupported command: {:02x}",
                        frame.command_id()
                    )));
                }
            }

            Ok(())
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            // Return None for now, later this will return clipboard updates
            Ok(None)
        }
    }

    /// Application service for launching applications
    #[derive(Debug)]
    pub struct AppService {
        /// Service name
        name: String,

        /// Service ID
        #[allow(dead_code)]
        id: Uuid,

        /// Sender for outgoing frames
        frame_sender: Sender<Frame>,

        /// Receiver for outgoing frames
        frame_receiver: Receiver<Frame>,

        /// Whether the service is running
        running: bool,
    }

    impl Default for AppService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AppService {
        /// Create a new application service
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel(100);

            Self {
                name: "app".to_string(),
                id: Uuid::new_v4(),
                frame_sender: tx,
                frame_receiver: rx,
                running: false,
            }
        }

        /// Launch an application based on the command
        async fn launch_application(&self, cmd: LaunchAppCommand) -> Result<()> {
            info!("Launching application: {}", cmd.application_path);

            // Check if this is a default application or custom path
            if cmd.application_path.starts_with("default:") {
                let app_type = cmd.application_path.trim_start_matches("default:");
                self.launch_default_app(app_type)
            } else {
                // Custom application path
                self.launch_custom_app(&cmd.application_path, cmd.args.as_deref())
            }
        }

        /// Launch a system default application based on type
        fn launch_default_app(&self, app_type: &str) -> Result<()> {
            match app_type {
                "notepad" => self.launch_notepad(),
                "textedit" => self.launch_textedit(),
                "calculator" => self.launch_calculator(),
                "browser" => self.launch_browser(),
                "terminal" => self.launch_terminal(),
                _ => Err(Error::Service(format!(
                    "Unknown default app type: {}",
                    app_type
                ))),
            }
        }

        /// Launch custom application with arguments
        fn launch_custom_app(&self, path: &str, args: Option<&str>) -> Result<()> {
            info!(
                "Launching custom application: {} with args: {:?}",
                path, args
            );

            let mut command = Command::new(path);

            if let Some(args_str) = args {
                // Split args by space, respecting quotes
                let mut args_vec = Vec::new();
                let mut current_arg = String::new();
                let mut in_quotes = false;

                for c in args_str.chars() {
                    match c {
                        '"' => in_quotes = !in_quotes,
                        ' ' if !in_quotes => {
                            if !current_arg.is_empty() {
                                args_vec.push(current_arg);
                                current_arg = String::new();
                            }
                        }
                        _ => current_arg.push(c),
                    }
                }

                if !current_arg.is_empty() {
                    args_vec.push(current_arg);
                }

                command.args(&args_vec);
            }

            match command.spawn() {
                Ok(_) => {
                    info!("Successfully launched application: {}", path);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to launch application: {}", e);
                    Err(Error::Service(format!(
                        "Failed to launch application: {}",
                        e
                    )))
                }
            }
        }

        /// Launch Windows Notepad
        #[cfg(target_os = "windows")]
        fn launch_notepad(&self) -> Result<()> {
            info!("Launching Notepad");
            match Command::new("notepad.exe").spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::Service(format!("Failed to launch Notepad: {}", e))),
            }
        }

        /// Launch macOS TextEdit
        #[cfg(target_os = "macos")]
        fn launch_textedit(&self) -> Result<()> {
            info!("Launching TextEdit");
            match Command::new("open").arg("-a").arg("TextEdit").spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::Service(format!("Failed to launch TextEdit: {}", e))),
            }
        }

        /// Launch Linux text editor (gedit, nano, etc.)
        #[cfg(target_os = "linux")]
        fn launch_textedit(&self) -> Result<()> {
            info!("Launching text editor");
            // Try several common editors
            for editor in &[
                "gedit", "kate", "kwrite", "mousepad", "leafpad", "nano", "vim", "vi",
            ] {
                match Command::new(editor).spawn() {
                    Ok(_) => return Ok(()),
                    Err(_) => continue,
                }
            }
            Err(Error::Service(
                "Failed to find a text editor to launch".to_string(),
            ))
        }

        /// Launch calculator application
        fn launch_calculator(&self) -> Result<()> {
            info!("Launching Calculator");

            #[cfg(target_os = "windows")]
            {
                match Command::new("calc.exe").spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!(
                        "Failed to launch Calculator: {}",
                        e
                    ))),
                }
            }

            #[cfg(target_os = "macos")]
            {
                match Command::new("open").arg("-a").arg("Calculator").spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!(
                        "Failed to launch Calculator: {}",
                        e
                    ))),
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Try several common calculators
                for calc in &["gnome-calculator", "kcalc", "xcalc", "qalculate"] {
                    match Command::new(calc).spawn() {
                        Ok(_) => return Ok(()),
                        Err(_) => continue,
                    }
                }
                Err(Error::Service(
                    "Failed to find a calculator to launch".to_string(),
                ))
            }

            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            {
                Err(Error::Service(
                    "Calculator launch not supported on this platform".to_string(),
                ))
            }
        }

        /// Launch web browser
        fn launch_browser(&self) -> Result<()> {
            info!("Launching Web Browser");

            #[cfg(target_os = "windows")]
            {
                match Command::new("explorer")
                    .arg("https://www.google.com")
                    .spawn()
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!("Failed to launch browser: {}", e))),
                }
            }

            #[cfg(target_os = "macos")]
            {
                match Command::new("open").arg("https://www.google.com").spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!("Failed to launch browser: {}", e))),
                }
            }

            #[cfg(target_os = "linux")]
            {
                match Command::new("xdg-open")
                    .arg("https://www.google.com")
                    .spawn()
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!("Failed to launch browser: {}", e))),
                }
            }

            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            {
                Err(Error::Service(
                    "Browser launch not supported on this platform".to_string(),
                ))
            }
        }

        /// Launch terminal emulator
        fn launch_terminal(&self) -> Result<()> {
            info!("Launching Terminal");

            #[cfg(target_os = "windows")]
            {
                match Command::new("cmd.exe").spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!("Failed to launch terminal: {}", e))),
                }
            }

            #[cfg(target_os = "macos")]
            {
                match Command::new("open").arg("-a").arg("Terminal").spawn() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::Service(format!("Failed to launch terminal: {}", e))),
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Try several common terminals
                for term in &["gnome-terminal", "konsole", "xterm", "rxvt", "terminator"] {
                    match Command::new(term).spawn() {
                        Ok(_) => return Ok(()),
                        Err(_) => continue,
                    }
                }
                Err(Error::Service(
                    "Failed to find a terminal to launch".to_string(),
                ))
            }

            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            {
                Err(Error::Service(
                    "Terminal launch not supported on this platform".to_string(),
                ))
            }
        }

        // Add stubs for methods that might not be needed on all platforms
        #[cfg(not(target_os = "windows"))]
        fn launch_notepad(&self) -> Result<()> {
            Err(Error::Service(
                "Notepad is only available on Windows".to_string(),
            ))
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        fn launch_textedit(&self) -> Result<()> {
            #[cfg(target_os = "windows")]
            {
                // Redirect to Notepad on Windows
                return self.launch_notepad();
            }

            #[cfg(not(target_os = "windows"))]
            {
                Err(Error::Service(
                    "TextEdit alternative not available on this platform".to_string(),
                ))
            }
        }
    }

    #[async_trait]
    impl Service for AppService {
        fn name(&self) -> &str {
            &self.name
        }

        async fn start(&mut self) -> Result<()> {
            if self.running {
                return Err(Error::Service("App service already running".to_string()));
            }

            self.running = true;
            info!("Application service started");
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            if !self.running {
                return Err(Error::Service("App service not running".to_string()));
            }

            self.running = false;
            info!("Application service stopped");
            Ok(())
        }

        async fn process_frame(&mut self, frame: &Frame) -> Result<()> {
            if !self.running {
                return Err(Error::Service("App service not running".to_string()));
            }

            match frame.command_id() {
                cmd if cmd == CommandId::LaunchApp as u8 => {
                    // Deserialize the command
                    let launch_cmd = serde_json::from_slice::<LaunchAppCommand>(frame.payload())
                        .map_err(|e| {
                            Error::Protocol(format!("Invalid LaunchApp command: {}", e))
                        })?;

                    info!("Received launch app request: {:?}", launch_cmd);

                    // Launch the application
                    self.launch_application(launch_cmd).await?;

                    // Send acknowledgement
                    let ack_frame = Frame::new(CommandId::Ack as u8, Vec::new());
                    self.frame_sender.send(ack_frame).await.map_err(|e| {
                        Error::Service(format!("Failed to send acknowledgement: {}", e))
                    })?;

                    Ok(())
                }
                _ => Err(Error::Protocol(format!(
                    "App service: Unsupported command: {:02x}",
                    frame.command_id()
                ))),
            }
        }

        async fn get_frame(&mut self) -> Result<Option<Frame>> {
            // App service doesn't generate frames except acknowledgements
            match self.frame_receiver.try_recv() {
                Ok(frame) => Ok(Some(frame)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(mpsc::error::TryRecvError::Disconnected) => Err(Error::Service(
                    "App service channel disconnected".to_string(),
                )),
            }
        }
    }

    /// Service factory for creating service instances
    pub struct ServiceFactory;

    impl ServiceFactory {
        /// Create a new service instance by name
        pub fn create(name: &str) -> Option<Box<dyn Service + Send>> {
            use self::services::{AppService, ClipboardService, DisplayService, InputService};

            match name {
                "display" => Some(Box::new(DisplayService::new())),
                "input" => Some(Box::new(InputService::new())),
                "clipboard" => Some(Box::new(ClipboardService::new())),
                "app" => Some(Box::new(AppService::new())),
                _ => None,
            }
        }

        /// Get a list of available service names
        #[allow(dead_code)]
        pub fn available_services() -> Vec<&'static str> {
            vec!["display", "input", "clipboard", "app"]
        }
    }
}
