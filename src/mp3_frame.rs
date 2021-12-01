use super::mp3_body::Mp3Body;
use super::mp3_header::{Mp3ChannelMode, Mp3FrameHeader};

pub struct Mp3Frame {
    pub header: Mp3FrameHeader,
    pub body: Mp3Body,
    pub position: u32,
}

impl Mp3Frame {
    pub fn new(header: Mp3FrameHeader, data: Vec<u8>, position: u32) -> Mp3Frame {
        let is_mono = header.channel_mode() == Mp3ChannelMode::SingleChannel;
        Mp3Frame {
            header: header,
            body: Mp3Body::new(data, is_mono),
            position: position,
        }
    }
}
