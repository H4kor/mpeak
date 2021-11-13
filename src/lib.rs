/**
 * Resources:
 * http://www.datavoyage.com/mpgscript/mpeghdr.htm
 * http://www.multiweb.cz/twoinches/mp3inside.htm
 * https://wiki.hydrogenaud.io/index.php?title=MP3
 */

mod mp3_header;

use std::io::prelude::*;
use std::fs::File;
use mp3_header::*;


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

}