use std::time::Instant;

use egui::{
    ClippedPrimitive, Context, Event, Key, Modifiers,
    PointerButton, Pos2, RawInput, Rect, Vec2, TexturesDelta,
};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct EguiSDL2 {
    context: Context,
    raw_input: RawInput,
    last_frame_time: Instant,
    textures: TexturesDelta,
}

impl EguiSDL2 {
    pub fn new() -> Self {
        let context = Context::default();
        let raw_input = RawInput::default();
        
        Self {
            context,
            raw_input,
            last_frame_time: Instant::now(),
            textures: TexturesDelta::default(),
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn begin_frame(&mut self, window: &Window) {
        let now = Instant::now();
        let dt = now - self.last_frame_time;
        self.last_frame_time = now;
        
        self.raw_input.predicted_dt = dt.as_secs_f32();
        self.raw_input.screen_rect = Some(Rect::from_min_size(
            Pos2::ZERO,
            Vec2::new(window.size().0 as f32, window.size().1 as f32),
        ));
        
        self.context.begin_frame(self.raw_input.take());
    }

    pub fn end_frame(&mut self) -> (Vec<ClippedPrimitive>, TexturesDelta) {
        let output = self.context.end_frame();
        let clipped_primitives = self.context.tessellate(output.shapes, 1.0);
        
        // Process textures
        self.textures.append(output.textures_delta);
        
        (clipped_primitives, self.textures.clone())
    }
    
    pub fn process_event(&mut self, event: &SdlEvent) {
        self.handle_event(event);
    }

    fn handle_event(&mut self, event: &SdlEvent) {
        match event {
            SdlEvent::MouseMotion { x, y, .. } => {
                self.raw_input.events.push(Event::PointerMoved(Pos2::new(*x as f32, *y as f32)));
            }
            SdlEvent::MouseButtonDown { mouse_btn, x, y, .. } => {
                self.raw_input.events.push(Event::PointerButton {
                    pos: Pos2::new(*x as f32, *y as f32),
                    button: match mouse_btn {
                        MouseButton::Left => PointerButton::Primary,
                        MouseButton::Right => PointerButton::Secondary,
                        MouseButton::Middle => PointerButton::Middle,
                        _ => PointerButton::Primary,
                    },
                    pressed: true,
                    modifiers: Modifiers::default(),
                });
            }
            SdlEvent::MouseButtonUp { mouse_btn, x, y, .. } => {
                self.raw_input.events.push(Event::PointerButton {
                    pos: Pos2::new(*x as f32, *y as f32),
                    button: match mouse_btn {
                        MouseButton::Left => PointerButton::Primary,
                        MouseButton::Right => PointerButton::Secondary,
                        MouseButton::Middle => PointerButton::Middle,
                        _ => PointerButton::Primary,
                    },
                    pressed: false,
                    modifiers: Modifiers::default(),
                });
            }
            SdlEvent::MouseWheel { x, y, .. } => {
                self.raw_input.events.push(Event::Scroll(Vec2::new(*x as f32 * 10.0, *y as f32 * 10.0)));
            }
            SdlEvent::TextInput { text, .. } => {
                self.raw_input.events.push(Event::Text(text.clone()));
            }
            SdlEvent::KeyDown { keycode: Some(keycode), keymod, repeat, .. } if !*repeat => {
                if let Some(key) = keycode_to_key(*keycode) {
                    self.raw_input.events.push(Event::Key {
                        key,
                        physical_key: Some(key),  // Use Some() to wrap the key
                        pressed: true,
                        repeat: *repeat,
                        modifiers: keymod_to_modifiers(*keymod),
                    });
                }
            }
            SdlEvent::KeyUp { keycode: Some(keycode), keymod, repeat, .. } => {
                if let Some(key) = keycode_to_key(*keycode) {
                    self.raw_input.events.push(Event::Key {
                        key,
                        physical_key: Some(key),  // Use Some() to wrap the key
                        pressed: false,
                        repeat: *repeat,
                        modifiers: keymod_to_modifiers(*keymod),
                    });
                }
            }
            _ => {}
        }
    }

    pub fn paint_primitives(&mut self, canvas: &mut Canvas<Window>, primitives: Vec<ClippedPrimitive>, textures_delta: TexturesDelta) {
        // Clear the canvas with a dark background color
        canvas.set_draw_color(sdl2::pixels::Color::RGB(30, 30, 30));
        canvas.clear();

        // Update textures if there are any changes
        if !textures_delta.is_empty() {
            // We would typically process texture updates here
            println!("Texture delta contains {} set operations and {} free operations", 
                textures_delta.set.len(), textures_delta.free.len());
        }
        
        // Improved implementation: render UI elements
        for primitive in &primitives {
            // Set up clipping rectangle
            let clip_rect = primitive.clip_rect;
            let min_x = clip_rect.min.x.round() as i32;
            let min_y = clip_rect.min.y.round() as i32;
            let width = (clip_rect.max.x - clip_rect.min.x).round() as u32;
            let height = (clip_rect.max.y - clip_rect.min.y).round() as u32;
            
            // Only render if clip rectangle is valid
            if width > 0 && height > 0 {
                // Set the clip rectangle
                let _ = canvas.set_clip_rect(sdl2::rect::Rect::new(min_x, min_y, width, height));
                
                // Process mesh based on its primitive type
                match &primitive.primitive {
                    egui::epaint::Primitive::Mesh(mesh) => {
                        if mesh.indices.len() >= 3 {
                            // Special case for rectangles (common UI elements)
                            if mesh.indices.len() == 6 {
                                // Check if this is likely a rectangle (6 indices = 2 triangles for a quad)
                                // For UI elements like windows, buttons, etc., this is a common pattern
                                
                                // Determine the bounds of this mesh (min/max x,y)
                                let mut min_x = f32::MAX;
                                let mut min_y = f32::MAX;
                                let mut max_x = f32::MIN;
                                let mut max_y = f32::MIN;
                                
                                for idx in &mesh.indices {
                                    let vertex = &mesh.vertices[*idx as usize];
                                    min_x = min_x.min(vertex.pos.x);
                                    min_y = min_y.min(vertex.pos.y);
                                    max_x = max_x.max(vertex.pos.x);
                                    max_y = max_y.max(vertex.pos.y);
                                }
                                
                                // Get the color (use the first vertex color)
                                let color = mesh.vertices[mesh.indices[0] as usize].color;
                                canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                                    color.r(), color.g(), color.b(), color.a()
                                ));
                                
                                // Draw as a filled rectangle
                                let rect = sdl2::rect::Rect::new(
                                    min_x as i32, 
                                    min_y as i32,
                                    (max_x - min_x) as u32, 
                                    (max_y - min_y) as u32
                                );
                                let _ = canvas.fill_rect(rect);
                            } else {
                                // For more complex shapes, draw individual triangles
                                for i in (0..mesh.indices.len()).step_by(3) {
                                    if i + 2 < mesh.indices.len() {
                                        let idx1 = mesh.indices[i] as usize;
                                        let idx2 = mesh.indices[i + 1] as usize;
                                        let idx3 = mesh.indices[i + 2] as usize;
                                        
                                        if idx1 < mesh.vertices.len() && idx2 < mesh.vertices.len() && idx3 < mesh.vertices.len() {
                                            // Get the vertices
                                            let v1 = &mesh.vertices[idx1];
                                            let v2 = &mesh.vertices[idx2];
                                            let v3 = &mesh.vertices[idx3];
                                            
                                            // Convert egui color to SDL color
                                            let color = v1.color;
                                            canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                                                color.r(), color.g(), color.b(), color.a()
                                            ));
                                            
                                            // Create polygon from points
                                            let points = [
                                                sdl2::rect::Point::new(v1.pos.x as i32, v1.pos.y as i32),
                                                sdl2::rect::Point::new(v2.pos.x as i32, v2.pos.y as i32),
                                                sdl2::rect::Point::new(v3.pos.x as i32, v3.pos.y as i32),
                                            ];
                                            
                                            // Draw the triangle using lines (simple approach)
                                            let _ = canvas.draw_line(points[0], points[1]);
                                            let _ = canvas.draw_line(points[1], points[2]);
                                            let _ = canvas.draw_line(points[2], points[0]);
                                            
                                            // For a filled look, we'll add some cross-hatching inside
                                            let center_x = (v1.pos.x + v2.pos.x + v3.pos.x) / 3.0;
                                            let center_y = (v1.pos.y + v2.pos.y + v3.pos.y) / 3.0;
                                            let center = sdl2::rect::Point::new(center_x as i32, center_y as i32);
                                            
                                            // Draw lines from center to each vertex
                                            let _ = canvas.draw_line(center, points[0]);
                                            let _ = canvas.draw_line(center, points[1]);
                                            let _ = canvas.draw_line(center, points[2]);
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Draw text (as colored rectangles for now - actual text rendering would need textures)
                        if !mesh.vertices.is_empty() && mesh.indices.is_empty() {
                            for vertex in &mesh.vertices {
                                // Draw a placeholder for each character
                                let color = vertex.color;
                                canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                                    color.r(), color.g(), color.b(), color.a()
                                ));
                                
                                let pos = vertex.pos;
                                let _ = canvas.fill_rect(sdl2::rect::Rect::new(
                                    pos.x as i32, 
                                    pos.y as i32, 
                                    4, 8
                                ));
                            }
                        }
                    },
                    egui::epaint::Primitive::Callback(_) => {
                        // Custom rendering callbacks not supported in this simple implementation
                        println!("Custom rendering callbacks not supported");
                    },
                }
            }
            
            // Reset the clipping rectangle
            let _ = canvas.set_clip_rect(None);
        }
        
        // Present the final rendered frame
        canvas.present();
    }
}

fn keycode_to_key(keycode: Keycode) -> Option<Key> {
    match keycode {
        Keycode::A => Some(Key::A),
        Keycode::B => Some(Key::B),
        Keycode::C => Some(Key::C),
        Keycode::D => Some(Key::D),
        Keycode::E => Some(Key::E),
        Keycode::F => Some(Key::F),
        Keycode::G => Some(Key::G),
        Keycode::H => Some(Key::H),
        Keycode::I => Some(Key::I),
        Keycode::J => Some(Key::J),
        Keycode::K => Some(Key::K),
        Keycode::L => Some(Key::L),
        Keycode::M => Some(Key::M),
        Keycode::N => Some(Key::N),
        Keycode::O => Some(Key::O),
        Keycode::P => Some(Key::P),
        Keycode::Q => Some(Key::Q),
        Keycode::R => Some(Key::R),
        Keycode::S => Some(Key::S),
        Keycode::T => Some(Key::T),
        Keycode::U => Some(Key::U),
        Keycode::V => Some(Key::V),
        Keycode::W => Some(Key::W),
        Keycode::X => Some(Key::X),
        Keycode::Y => Some(Key::Y),
        Keycode::Z => Some(Key::Z),
        Keycode::Num0 => Some(Key::Num0),
        Keycode::Num1 => Some(Key::Num1),
        Keycode::Num2 => Some(Key::Num2),
        Keycode::Num3 => Some(Key::Num3),
        Keycode::Num4 => Some(Key::Num4),
        Keycode::Num5 => Some(Key::Num5),
        Keycode::Num6 => Some(Key::Num6),
        Keycode::Num7 => Some(Key::Num7),
        Keycode::Num8 => Some(Key::Num8),
        Keycode::Num9 => Some(Key::Num9),
        Keycode::Space => Some(Key::Space),
        Keycode::Tab => Some(Key::Tab),
        Keycode::Return => Some(Key::Enter),
        Keycode::Escape => Some(Key::Escape),
        Keycode::Backspace => Some(Key::Backspace),
        Keycode::Up => Some(Key::ArrowUp),
        Keycode::Down => Some(Key::ArrowDown),
        Keycode::Left => Some(Key::ArrowLeft),
        Keycode::Right => Some(Key::ArrowRight),
        Keycode::Insert => Some(Key::Insert),
        Keycode::Delete => Some(Key::Delete),
        Keycode::Home => Some(Key::Home),
        Keycode::End => Some(Key::End),
        Keycode::PageUp => Some(Key::PageUp),
        Keycode::PageDown => Some(Key::PageDown),
        _ => None,
    }
}

fn keymod_to_modifiers(keymod: Mod) -> Modifiers {
    Modifiers {
        alt: keymod.contains(Mod::LALTMOD) || keymod.contains(Mod::RALTMOD),
        ctrl: keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD),
        shift: keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD),
        command: keymod.contains(Mod::LGUIMOD) || keymod.contains(Mod::RGUIMOD),
        ..Default::default()
    }
}
