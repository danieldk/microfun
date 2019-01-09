use microbit::hal::delay::Delay;
use microbit::led;

pub trait ScrollImages {
    /// Display the images scrolling from left to right.
    ///
    /// `images` should be a concatenation of the rows of the images.
    fn scroll_images(&mut self, delay: &mut Delay, images: &[[u8; 5]], frame_ms: u32);
}

impl ScrollImages for led::Display {
    fn scroll_images(&mut self, delay: &mut Delay, images: &[[u8; 5]], frame_ms: u32) {
        let mut image = [[0; 5]; 5];

        for offset in 0..(images.len() - 5) {
            for row in 0..5 {
                for col in 0..5 {
                    image[row][col] = images[row + floor(col + offset, 5)][(col + offset) % 5];
                }
            }

            self.display(delay, image, frame_ms);
        }
    }
}

fn floor(n: usize, multiple: usize) -> usize {
    n - n % multiple
}
