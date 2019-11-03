use microbit::hal::delay::Delay;
use microbit::led;

use crate::image::Image;

pub trait ScrollImages {
    /// Display the images scrolling from left to right.
    ///
    /// `images` should be a concatenation of the rows of the images.
    fn scroll_images(&mut self, delay: &mut Delay, images: &[Image], frame_ms: u32);
}

impl ScrollImages for led::Display {
    fn scroll_images(&mut self, delay: &mut Delay, images: &[Image], frame_ms: u32) {
        let mut image = [[0; 5]; 5];

        for offset in 0..((images.len() * 5) - 5) {
            for (row, image_row) in image.iter_mut().enumerate() {
                for (col, image_col) in image_row.iter_mut().enumerate().take(5) {
                    let col_offset = col + offset;
                    *image_col = images[col_offset / 5][row][col_offset % 5];
                }
            }

            self.display(delay, image, frame_ms);
        }
    }
}
