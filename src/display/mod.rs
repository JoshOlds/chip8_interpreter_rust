pub mod crossterm_display;

#[derive(Copy, Clone)]
pub enum DisplayMode {
    H64V32MONOCHROME,
    H128V64MONOCHROME,
}

impl DisplayMode {
    /// Returns the Horizontal Resolution, in pixels, of this DisplayMode
    pub fn get_h_res(&self) -> i32 {
        match *self {
            DisplayMode::H64V32MONOCHROME => 64,
            DisplayMode::H128V64MONOCHROME => 128,
        }
    }

    /// Returns the Vertical Resolution, in pixels, of this DisplayMode
    pub fn get_v_res(&self) -> i32 {
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
    fn set_display_mode(&mut self, mode: DisplayMode);
    /// Gets a reference to the DisplayBuffer
    fn get_display_buffer(&mut self) -> &mut DisplayBuffer;
    /// Draws the display buffer to the screen
    fn draw(&mut self);
    /// Clears the display (not the display buffer)
    fn clear_screen(&mut self);
    /// Hides the display
    fn hide(&mut self);
}

/// Wrapper for Display buffer memory. Includes helper methods for working with display memory.
#[derive(Copy, Clone)]
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

    /// Clears the display buffer (sets all values to false)
    pub fn clear(&mut self) {
        self.buff = [[false; 128]; 64];
    }

    /// Sets a pixel on the DisplayBuffer to ON (true). If the pixel being set is already on (true),
    /// then a collision has occurred. Settings pixels off-screen (outside resolution bounds), will
    /// cause the pixel coordinates to roll-over.
    /// Returns true when a collision has occurred.
    pub fn set_pixel(&mut self, x: i32, y: i32) -> bool {
        // Normalize (roll over) pixel values into resolution range
        let mut x = x % self.display_mode.get_h_res();
        let mut y = y % self.display_mode.get_v_res();
        if x < 0 {
            x += self.display_mode.get_h_res();
        }
        if y < 0 {
            y += self.display_mode.get_v_res();
        }

        let pixel: &mut bool = &mut self.buff[y as usize][x as usize];
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

    pub fn write_sprite(&mut self, x: i32, y: i32, sprite_slice: &[u8]) -> bool {
        let mut collision = false;

        for (i, byte) in sprite_slice.iter().enumerate() {
            let i = i as i32;

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
