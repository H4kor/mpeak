/**
 * Resources:
 * http://www.datavoyage.com/mpgscript/mpeghdr.htm
 * http://www.multiweb.cz/twoinches/mp3inside.htm
 * https://wiki.hydrogenaud.io/index.php?title=MP3
 */

use std::io::prelude::*;
use std::fs::File;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MPeakError {
    CannotOpenFile,
    CannotReadFile,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Mp3Version{
    V25 = 0,
    Reserved = 1,
    V2 = 2,
    V1 = 3
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Mp3Layer {
    Reserved = 0,
    Layer3 = 1,
    Layer2 = 2,
    Layer1 = 3
}

#[derive(Debug, PartialEq)]
pub enum Mp3Protection {
    ProtectedByCrc,
    NotProtected,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Mp3ChannelMode {
    Stereo = 0,
    JointStereo = 1,
    DualChannel = 2,
    SingleChannel = 3,
}


#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Mp3Emphasis {
    None = 1,
    _50_15Ms = 2,
    Reserved = 3,
    CcitJ17 = 4,
}

pub struct Mp3FrameHeader {
    //            AAAAAAAA AAABBCCD EEEEFFGH IIJJKLMM
    // sync:      11111111 11100000 00000000 00000000
    // version:   00000000 00011000 00000000 00000000
    // layer:     00000000 00000110 00000000 00000000
    // protected: 00000000 00000001 00000000 00000000
    // bitrate:   00000000 00000000 11110000 00000000
    // sampling:  00000000 00000000 00001100 00000000
    // padding:   00000000 00000000 00000010 00000000
    // private:   00000000 00000000 00000001 00000000
    // channel:   00000000 00000000 00000000 11000000
    // mode ext:  00000000 00000000 00000000 00110000
    // copyright: 00000000 00000000 00000000 00001000
    // original:  00000000 00000000 00000000 00000100
    // emphasis:  00000000 00000000 00000000 00000011
    data: u32
}


const V1_L1: [u16; 16] = [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0];
const V1_L2: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0];
const V1_L3: [u16; 16] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0];
const V2_L1: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0];
const V2_L2_3: [u16; 16] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0];
const VR_LR: [u16; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const MPEG1: [u16; 4] = [ 44100, 48000, 32000, 0 ];
const MPEG2: [u16; 4] = [ 22050, 24000, 16000, 0 ];
const MPEG2_5: [u16; 4] = [ 11025, 12000, 8000, 0 ];
const MPEGR: [u16; 4] = [ 0, 0, 0, 0 ];

impl Mp3FrameHeader {
    pub fn new(data: u32) -> Mp3FrameHeader {
        Mp3FrameHeader{
            data: data
        }
    }

    pub fn version(&self) -> Mp3Version {
        let bits: (bool, bool) = (
            self.data >> 20 & 1 == 1,
            self.data >> 19 & 1 == 1
        );
        match bits {
            (false, false) => Mp3Version::V25,
            (false, true) => Mp3Version::Reserved,
            (true, false) => Mp3Version::V2,
            (true, true) => Mp3Version::V1,
        }
    }


    pub fn layer(&self) -> Mp3Layer {
        let bits: (bool, bool) = (
            self.data >> 18 & 1 == 1,
            self.data >> 17 & 1 == 1
        );
        match bits {
            (false, false) => Mp3Layer::Reserved,
            (false, true) => Mp3Layer::Layer3,
            (true, false) => Mp3Layer::Layer2,
            (true, true) => Mp3Layer::Layer1,
        }
    }

    pub fn protected(&self) -> Mp3Protection {
        if self.data >> 16 & 1 == 1 {
            Mp3Protection::NotProtected
        } else {
            Mp3Protection::ProtectedByCrc
        }
    }

    pub fn bitrate_index(&self) -> u8 {
        (self.data >> 12 & 0xf) as u8
    }

    pub fn sampling_rate_index(&self) -> u8 {
        (self.data >> 10 & 0x3) as u8
    }
    
    pub fn padding_bit(&self) -> bool {
        self.data >> 9 & 1 == 1
    }
    
    pub fn private_bit(&self) -> bool {
        self.data >> 8 & 1 == 1
    }

    pub fn channel_mode(&self) -> Mp3ChannelMode {
        let bits: (bool, bool) = (
            self.data >> 7 & 1 == 1,
            self.data >> 6 & 1 == 1
        );
        match bits {
            (false, false) => Mp3ChannelMode::Stereo,
            (false, true) => Mp3ChannelMode::JointStereo,
            (true, false) => Mp3ChannelMode::DualChannel,
            (true, true) => Mp3ChannelMode::SingleChannel,
        }
    }

    pub fn mode_extension(&self) -> u8 {
        (self.data >> 4 & 0x3) as u8
    }


    pub fn copyright(&self) -> bool {
        self.data >> 3 & 1 == 1
    }

    pub fn original(&self) -> bool {
        self.data >> 2 & 1 == 1
    }

    pub fn emphasis(&self) -> Mp3Emphasis {
        let bits: (bool, bool) = (
            self.data >> 1 & 1 == 1,
            self.data >> 0 & 1 == 1
        );
        match bits {
            (false, false) => Mp3Emphasis::None,
            (false, true) => Mp3Emphasis::_50_15Ms,
            (true, false) => Mp3Emphasis::Reserved,
            (true, true) => Mp3Emphasis::CcitJ17,
        }
    }

    pub fn frame_length(&self) -> usize {
        let bitrate_list = match self.version() {
            Mp3Version::V1 => match self.layer() {
                Mp3Layer::Layer1 => V1_L1,
                Mp3Layer::Layer2 => V1_L2,
                Mp3Layer::Layer3 => V1_L3,
                Mp3Layer::Reserved => VR_LR
            },
            Mp3Version::V25 | Mp3Version::V2 =>  match self.layer() {
                Mp3Layer::Layer1 => V2_L1,
                Mp3Layer::Layer2 | Mp3Layer::Layer3 => V2_L2_3,
                Mp3Layer::Reserved => VR_LR
            },
            Mp3Version::Reserved => VR_LR
        };
        let bitrate = bitrate_list[self.bitrate_index() as usize];

        let sample_rate_list = match self.version() {
            Mp3Version::V1 => MPEG1,
            Mp3Version::V2 => MPEG2,
            Mp3Version::V25 => MPEG2_5,
            Mp3Version::Reserved => MPEGR,
        };
        let sample_rate = sample_rate_list[self.sampling_rate_index() as usize];
        let frame_len: u32 = 144 * (bitrate as u32) * 1000 / (sample_rate as u32);
        if self.padding_bit() {
            (frame_len + 1) as usize
        } else {
            frame_len as usize
        }
    }
}


impl fmt::Debug for Mp3FrameHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mp3FrameHeader")
            .field("version", &self.version())
            .field("layer", &self.layer())
            .finish()
    }
}

pub fn load_file(file_path: &String) -> Result<Vec<u8>, MPeakError> {
    match File::open(file_path) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => Ok(buffer),
                Err(_) => Err(MPeakError::CannotReadFile)
            }
        },
        Err(_) => Err(MPeakError::CannotOpenFile)
    }
}

/// Checks wether the data represents a mp3 file.
pub fn is_mp3_file(data: &Vec<u8>) -> bool {
    // check magic bytes for mp3
    if data.len() < 2 {
        // must have two bytes
        false
    } else if data[0] == 0xFF {
        // no id3
        data[1] == 0xFB || data[1] == 0xF3 || data[1] == 0xF2
    } else if data.len() < 3 {
        false
    } else {
        // id3
        data[0] == 0x49 && data[1] == 0x44 && data[2] == 0x33
    }
}

/// Check wether the data includes an id3 header
pub fn has_id3(data: &Vec<u8>) -> bool {
    data.len() > 2 && data[0] == 0x49 && data[1] == 0x44 && data[2] == 0x33
}

/// Retrieve the offset of the id3 data block
pub fn get_id3_offset(data: &Vec<u8>) -> u32 {
    if has_id3(data) {
        if data.len() < 10 {
            data.len() as u32
        } else {
            let slice: [u8; 4] = data[6..10].try_into().unwrap();
            10 + u32::from_be_bytes(slice)
        }
    } else {
        0
    }
}

/// Retrieve the id3 data block
pub fn get_id3_data(data: &Vec<u8>) -> Vec<u8> {
    let offset = get_id3_offset(data);
    data[0..offset as usize].to_vec()
}



pub fn get_first_mp3_frame_header(data: &Vec<u8>) -> Mp3FrameHeader {
    let offset = get_id3_offset(data) as usize;
    let header_data = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
    Mp3FrameHeader::new(header_data)
}

pub fn all_headers(data: &Vec<u8>) -> Vec<Mp3FrameHeader> {
    let mut offset = get_id3_offset(data) as usize;
    let mut headers = Vec::<Mp3FrameHeader>::new();
    while offset < data.len() {
        let header_data = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
        let curr = Mp3FrameHeader::new(header_data);
        offset += curr.frame_length();
        headers.push(curr);
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mp3_file_empty() {
        assert_eq!(is_mp3_file(&vec![]), false);
    }

    #[test]
    fn test_is_mp3_file_no_id3() {
        assert_eq!(is_mp3_file(&vec![ 0xFF ]), false);
        assert_eq!(is_mp3_file(&vec![ 0xFF, 0xFB ]), true);
    }

    #[test]
    fn test_is_mp3_file_id3() {
        assert_eq!(is_mp3_file(&vec![ 0x49 ]), false);
        assert_eq!(is_mp3_file(&vec![ 0x49, 0x44 ]), false);
        assert_eq!(is_mp3_file(&vec![ 0x49, 0x44, 0x33 ]), true);
    }

    #[test]
    fn test_has_id3() {
        assert_eq!(is_mp3_file(&vec![ 0x49 ]), false);
        assert_eq!(is_mp3_file(&vec![ 0x49, 0x44 ]), false);
        assert_eq!(is_mp3_file(&vec![ 0x49, 0x44, 0x33 ]), true);
        assert_eq!(is_mp3_file(&vec![ 0x49, 0x44, 0x33, 0x90 ]), true);
    }

    #[test]
    fn test_frame_header_version() {
        let header = Mp3FrameHeader::new(0b00000000000_00_0000000000000000000);
        assert_eq!(header.version(), Mp3Version::V25);
        let header = Mp3FrameHeader::new(0b00000000000_01_0000000000000000000);
        assert_eq!(header.version(), Mp3Version::Reserved);
        let header = Mp3FrameHeader::new(0b00000000000_10_0000000000000000000);
        assert_eq!(header.version(), Mp3Version::V2);
        let header = Mp3FrameHeader::new(0b00000000000_11_0000000000000000000);
        assert_eq!(header.version(), Mp3Version::V1);
    }


    #[test]
    fn test_frame_header_layer() {
        let header = Mp3FrameHeader::new(0b0000000000000_00_00000000000000000);
        assert_eq!(header.layer(), Mp3Layer::Reserved);
        let header = Mp3FrameHeader::new(0b0000000000000_01_00000000000000000);
        assert_eq!(header.layer(), Mp3Layer::Layer3);
        let header = Mp3FrameHeader::new(0b0000000000000_10_00000000000000000);
        assert_eq!(header.layer(), Mp3Layer::Layer2);
        let header = Mp3FrameHeader::new(0b0000000000000_11_00000000000000000);
        assert_eq!(header.layer(), Mp3Layer::Layer1);
    }

    #[test]
    fn test_frame_header_protected() {
        let header = Mp3FrameHeader::new(0b_00000000_00000001_00000000_00000000);
        assert_eq!(header.protected(), Mp3Protection::NotProtected);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.protected(), Mp3Protection::ProtectedByCrc);
    }

    #[test]
    fn test_frame_header_bitrate_index() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.bitrate_index(), 0);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_11110000_00000000);
        assert_eq!(header.bitrate_index(), 15);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_11000000_00000000);
        assert_eq!(header.bitrate_index(), 12);
    }

    #[test]
    fn test_frame_header_sampling_rate_index() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.sampling_rate_index(), 0);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000100_00000000);
        assert_eq!(header.sampling_rate_index(), 1);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00001000_00000000);
        assert_eq!(header.sampling_rate_index(), 2);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00001100_00000000);
        assert_eq!(header.sampling_rate_index(), 3);
    }

    #[test]
    fn test_frame_header_padding_bit() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000010_00000000);
        assert_eq!(header.padding_bit(), true);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.padding_bit(), false);
    }

    #[test]
    fn test_frame_header_private_bit() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000001_00000000);
        assert_eq!(header.private_bit(), true);
    }

    #[test]
    fn test_frame_header_channel_mode() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.channel_mode(), Mp3ChannelMode::Stereo);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_01000000);
        assert_eq!(header.channel_mode(), Mp3ChannelMode::JointStereo);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_10000000);
        assert_eq!(header.channel_mode(), Mp3ChannelMode::DualChannel);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_11000000);
        assert_eq!(header.channel_mode(), Mp3ChannelMode::SingleChannel);
    }

    #[test]
    fn test_frame_header_mode_extension() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.mode_extension(), 0);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00010000);
        assert_eq!(header.mode_extension(), 1);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00100000);
        assert_eq!(header.mode_extension(), 2);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00110000);
        assert_eq!(header.mode_extension(), 3);
    }

    #[test]
    fn test_frame_header_copyright() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00001000);
        assert_eq!(header.copyright(), true);
    }

    #[test]
    fn test_frame_header_original() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000100);
        assert_eq!(header.original(), true);
    }
    
    #[test]
    fn test_frame_header_emphasis() {
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000000);
        assert_eq!(header.emphasis(), Mp3Emphasis::None);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000001);
        assert_eq!(header.emphasis(), Mp3Emphasis::_50_15Ms);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000010);
        assert_eq!(header.emphasis(), Mp3Emphasis::Reserved);
        let header = Mp3FrameHeader::new(0b_00000000_00000000_00000000_00000011);
        assert_eq!(header.emphasis(), Mp3Emphasis::CcitJ17);
    }

    #[test]
    fn test_frame_length() {
        // V1 Layer 1 birate 4 samplerate 0
        let header = Mp3FrameHeader::new(0b_00000000_00011110_01000000_00000000);
        assert_eq!(header.frame_length(), 417);
        // V1 Layer 2 birate 13 samplerate 1
        let header = Mp3FrameHeader::new(0b_00000000_00011100_11010100_00000000);
        assert_eq!(header.frame_length(), 960);

        // V2 Layer 1 birate 2 samplerate 2
        let header = Mp3FrameHeader::new(0b_00000000_00010110_00101000_00000000);
        assert_eq!(header.frame_length(), 432);
        // V2 layer 3 birate 8 samplerate 2
        let header = Mp3FrameHeader::new(0b_00000000_00010010_10001000_00000000);
        assert_eq!(header.frame_length(), 576);

        // V2 layer 3 birate 8 samplerate 2 + padding
        let header = Mp3FrameHeader::new(0b_00000000_00010010_10001010_00000000);
        assert_eq!(header.frame_length(), 577);
    }

}