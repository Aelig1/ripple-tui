use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// A rippling body of water.
#[derive(Debug)]
pub struct Pond {
    /// The aspect ratio of a terminal character/cell. Height divided by width.
    ///
    /// Defaults to `2.0`.
    pub cell_aspect: f64,

    /// How quickly ripples fade. Expected to be between `0.0` and `1.0`.
    ///
    /// Lower values fade faster. `1.0` applies no damping.
    /// Defaults to `0.98`.
    pub damping: f64,

    // The size of the simulated area.
    width: usize,
    height: usize,

    // Wave states for the current and previous simulation ticks.
    current_buffer: Vec<f64>,
    previous_buffer: Vec<f64>,
}

impl Pond {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resizes the simulated area, clearing existing ripples if its size changes.
    pub fn resize(&mut self, width: u16, height: u16) {
        let width = width as usize;
        let height = height as usize;

        if width == self.width && height == self.height {
            return;
        }

        self.width = width;
        self.height = height;

        // Allocate extra padding to each edge.
        let len = width * height;
        self.current_buffer = vec![0.0; len];
        self.previous_buffer = vec![0.0; len];
    }

    /// Drops a droplet at the given terminal-cell coordinates to start a ripple.
    pub fn droplet(&mut self, x: u16, y: u16) {
        let x = x as usize;
        let y = y as usize;

        if x < self.width && y < self.height {
            let index = self.index(x, y);
            self.current_buffer[index] = -1.0;
        }
    }

    /// Advances the ripple simulation by one frame.
    pub fn tick(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.index(x, y);
                // Write new value to back buffer
                self.previous_buffer[i] = self.next_value(x, y);
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.current_buffer, &mut self.previous_buffer);
    }

    /// Returns the buffer index for the given coordinates.
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Returns the current wave value at coordinates, or `0.0` outside the simulated area.
    fn value(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }

        let i = self.index(x, y);
        self.current_buffer[i]
    }

    /// Returns the previous wave value at coordinates, or `0.0` outside the simulated area.
    fn previous_value(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }

        let i = self.index(x, y);
        self.previous_buffer[i]
    }

    /// Calculates the next wave value for the next frame at the given coordinates.
    fn next_value(&self, x: usize, y: usize) -> f64 {
        // Use four-neighbor von Neumann stencil for wave propagation.
        let n = y.checked_sub(1).map_or(0.0, |y| self.value(x, y));
        let w = x.checked_sub(1).map_or(0.0, |x| self.value(x, y));
        let s = y.checked_add(1).map_or(0.0, |y| self.value(x, y));
        let e = x.checked_add(1).map_or(0.0, |x| self.value(x, y));

        let next_value = (n + w + s + e) * 0.5 - self.previous_value(x, y);
        next_value * self.damping
    }
}

impl Default for Pond {
    fn default() -> Self {
        Self {
            cell_aspect: 2.0,
            damping: 0.98,
            width: 0,
            height: 0,
            current_buffer: Vec::default(),
            previous_buffer: Vec::default(),
        }
    }
}

impl Widget for &Pond {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..area.height {
            for x in 0..area.width {
                let value = self.value(x as usize, y as usize);
                let glyph = shade(value);
                buf[(area.x + x, area.y + y)].set_char(glyph);
            }
        }
    }
}

/// Converts a wave value to a shade character.
fn shade(value: f64) -> char {
    const RAMP: [char; 5] = [' ', '░', '▒', '▓', '█'];

    // Apply gain
    const GAIN: f64 = 100.0;
    let value = value * GAIN;

    let level = (value + (RAMP.len() as f64 / 2.0)).floor() as usize;
    RAMP[level.clamp(0, RAMP.len() - 1)]
}
