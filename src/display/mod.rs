pub mod crossterm_display;

#[derive(Copy, Clone)]
pub enum DisplayMode {
    H64V32MONOCHROME,
    H128V64MONOCHROME,
}

impl DisplayMode {
    /// Returns the Horizontal Resolution, in pixels, of this DisplayMode
    pub fn get_h_res(&self) -> u16 {
        match *self {
            DisplayMode::H64V32MONOCHROME => 64,
            DisplayMode::H128V64MONOCHROME => 128,
        }
    }

    /// Returns the Vertical Resolution, in pixels, of this DisplayMode
    pub fn get_v_res(&self) -> u16 {
        match *self {
            DisplayMode::H64V32MONOCHROME => 32,
            DisplayMode::H128V64MONOCHROME => 64,
        }
    }
}

/// CHIP-8 Display interface
pub trait Display {
    /// Returns the current DisplayMode of this Display
    fn get_display_mode(&self) -> DisplayMode;
    /// Sets the current DisplayMode of the Display
    fn set_display_mode(&self, mode: DisplayMode);
    /// Draws the display buffer to the screen
    fn draw(&self);
    /// Clears the display (not the display buffer)
    fn clear_screen(&self);
    /// Clears the display buffer (not the screen)
    fn clear_display_buffer(&self);
    /// Hides the display
    fn hide(&self);
    /// Gets a reference to the DisplayBuffer of this Display
    fn get_buffer(&self) -> &DisplayBuffer;
}

/// Wrapper for Display buffer memory. Includes helper methods for working with display memory.
pub struct DisplayBuffer {
    /// 2D array that matches size of the screen resolution.
    /// First dimension (ie. [buff[y]] ) is the row (vertical dimension)
    /// Second dimension (ie. [buff[y][x] ) is the column (horizontal dimension)
    buff: [[bool; 128]; 64],

    /// The DisplayMode of this DisplayBuffer
    display_mode: DisplayMode,
}

impl DisplayBuffer {
    /// Constructs a new DisplayBuffer
    pub fn new(mode: DisplayMode) -> DisplayBuffer {
        DisplayBuffer {
            buff: [[false; 128]; 64],
            display_mode: mode,
        }
    }

    /// Sets a pixel on the DisplayBuffer to ON (true). If the pixel being set is already on (true),
    /// then a collision has occurred. Settings pixels off-screen (outside resolution bounds), will
    /// cause the pixel coordinates to roll-over.
    /// Returns true when a collision has occurred.
    pub fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        // Normalize (roll over) pixel values into resolution range
        let x = x % self.display_mode.get_h_res();
        let y = y % self.display_mode.get_v_res();

        let pixel: &mut bool = &mut self.buff[y][x];
        // Pixel is already on
        return if *pixel {
            *pixel = false;
            true
        }
        // Pixel is off
        else {
            *pixel = true;
            false
        };
    }

    pub fn write_sprite(&mut self, x: u8, y: u8, sprite_slice: &[u8]) -> bool {
        let mut collision = false;

        for (i, byte) in sprite_slice.iter().enumerate() {
            if byte & 0x80 != 0 {
                if self.set_pixel(x, y + i) {
                    collision = true;
                }
            }
            if byte & 0x40 != 0 {
                if self.set_pixel(x + 1, y + i) {
                    collision = true;
                }
            }
            if byte & 0x20 != 0 {
                if self.set_pixel(x + 2, y + i) {
                    collision = true;
                }
            }
            if byte & 0x10 != 0 {
                if self.set_pixel(x + 3, y + i) {
                    collision = true;
                }
            }
            if byte & 0x08 != 0 {
                if self.set_pixel(x + 4, y + i) {
                    collision = true;
                }
            }
            if byte & 0x04 != 0 {
                if self.set_pixel(x + 5, y + i) {
                    collision = true;
                }
            }
            if byte & 0x02 != 0 {
                if self.set_pixel(x + 6, y + i) {
                    collision = true;
                }
            }
            if byte & 0x01 != 0 {
                if self.set_pixel(x + 7, y + i) {
                    collision = true;
                }
            }
        }
        collision
    }
}
