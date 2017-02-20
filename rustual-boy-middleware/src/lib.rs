extern crate rustual_boy_core;

mod color;
mod color_frame;
mod anaglyphizer;
mod gamma_adjust_sink;
mod most_recent_sink;
mod side_by_side_stereoscopy;

// reexports
pub use color::Color;
pub use color_frame::ColorFrame;
pub use anaglyphizer::Anaglyphizer;
pub use gamma_adjust_sink::GammaAdjustSink;
pub use most_recent_sink::MostRecentSink;
pub use side_by_side_stereoscopy::SideBySideStereoscopy;
