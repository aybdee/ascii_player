use ascii::AsciiImage;

pub trait FrameSource {
    fn get_frames() -> AsciiImage;
}
