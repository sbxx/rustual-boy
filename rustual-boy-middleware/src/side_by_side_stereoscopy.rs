use color::Color;
use color_frame::ColorFrame;
use rustual_boy_core::sinks::{Sink, VideoFrame};
use rustual_boy_core::vip::{DISPLAY_PIXELS, DISPLAY_RESOLUTION_X, DISPLAY_RESOLUTION_Y};
use std::ptr::copy_nonoverlapping;
use std::mem;

// To use this, it is currently required to double the
// emulator window width and the DISPLAY_RESOLUTION_X const 

/// A utility for the Rustual Boy core that places the left
/// and right frame side by side.
pub struct SideBySideStereoscopy<T: Sink<ColorFrame>> {
    /// Sink to which we push our frame as they come in
    inner: T,
}

impl<T: Sink<ColorFrame>> SideBySideStereoscopy<T> {
    pub fn new(inner: T) -> SideBySideStereoscopy<T> {
        SideBySideStereoscopy { inner: inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Sink<ColorFrame>> Sink<VideoFrame> for SideBySideStereoscopy<T> {
    fn append(&mut self, frame: VideoFrame) {
        let mut output: Vec<Color> = Vec::new();
        output.reserve_exact(DISPLAY_PIXELS);
        let (ref l_frame, ref r_frame) = frame;

        unsafe {
            let l_buffer = l_frame.as_ptr();
            let r_buffer = r_frame.as_ptr();
            {
                let o_ptr = output.as_mut_ptr();

                let half_diplay_resolution_x = (DISPLAY_RESOLUTION_X / 2) as isize;
                for y in 0..(DISPLAY_RESOLUTION_Y as isize) {
                    // left side
                    for x in 0..(half_diplay_resolution_x as isize) {
                        let col = *(r_buffer.offset(x + y * DISPLAY_RESOLUTION_X as isize));
                        let col: Color = (col, col, col).into();
                        *(o_ptr.offset(x + y * DISPLAY_RESOLUTION_X as isize)) = col;
                    }
                    // right side
                    for x in 0..(half_diplay_resolution_x as isize) {
                        let col = *(l_buffer.offset((x + y * DISPLAY_RESOLUTION_X as isize)));
                        let col: Color = (col, col, col).into();
                        *(o_ptr.offset((x + half_diplay_resolution_x) + y * DISPLAY_RESOLUTION_X as isize)) = col;
                    }
                }
                
                output.set_len(DISPLAY_PIXELS);
            }
            self.inner.append(output.into_boxed_slice());
        }
    }
}
