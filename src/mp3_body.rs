// References
// http://www.mp3-tech.org/programmer/docs/mp3_theory.pdf
// https://www.diva-portal.org/smash/get/diva2:830195/FULLTEXT01.pdf

pub struct Mp3Body {
    pub data: Vec<u8>,
    pub is_mono: bool,
}

impl Mp3Body {
    pub fn new(data: Vec<u8>, is_mono: bool) -> Mp3Body {
        Mp3Body {
            data: data,
            is_mono: is_mono,
        }
    }

    pub fn main_data_begin(&self) -> u16 {
        (self.data[0] as u16) << 1 | ((self.data[1] >> 7) as u16)
    }

    pub fn private_bits(&self) -> u8 {
        if self.is_mono {
            (self.data[1] & 0b_0111_1100) >> 2
        } else {
            (self.data[1] & 0b_0111_0000) >> 4
        }
    }

    pub fn scfsi(&self) -> u8 {
        if self.is_mono {
            (self.data[1] & 0b_0000_0011) << 2 | (self.data[2] & 0b_1100_0000) >> 6
        } else {
            (self.data[1] & 0b_0000_1111) << 4 | (self.data[2] & 0b_1111_0000) >> 4
        }
    }

    pub fn part2_3_length(&self) -> u32 {
        if self.is_mono {
            (self.data[2] as u32 & 0b_0011_1111) << 6 | (self.data[3] as u32 & 0b_1111_1100) >> 2
        } else {
            (self.data[2] as u32 & 0b_0000_1111) << 20
                | (self.data[3] as u32) << 12
                | (self.data[4] as u32) << 4
                | (self.data[5] as u32 & 0b_1111_0000) >> 4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_data_begin() {
        let data = vec![0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.main_data_begin(), 0);

        let data = vec![0b_1000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.main_data_begin(), 0b1_0000_0000);

        let data = vec![0b_0000_0000, 0b_1000_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.main_data_begin(), 1);
    }

    #[test]
    fn test_private_bits_mono() {
        let data = vec![0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.private_bits(), 0);

        let data = vec![0b_0000_0000, 0b_0000_0100, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.private_bits(), 1);

        let data = vec![0b_0000_0000, 0b_0100_0000, 0b_1000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.private_bits(), 16);
    }

    #[test]
    fn test_private_bits_stereo() {
        let data = vec![0b_0000_0000, 0b_1000_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.private_bits(), 0);

        let data = vec![0b_0000_0000, 0b_1001_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.private_bits(), 1);

        let data = vec![0b_0000_0000, 0b_1100_0000, 0b_0000_0000, 0b_0100_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.private_bits(), 4);
    }

    #[test]
    fn test_scfsi_mono() {
        let data = vec![0b_1111_1111, 0b_1111_1100, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.scfsi(), 0);

        let data = vec![0b_1111_1111, 0b_1111_1100, 0b_0100_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.scfsi(), 1);

        let data = vec![0b_1111_1111, 0b_1111_1100, 0b_1000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.scfsi(), 2);

        let data = vec![0b_1111_1111, 0b_1111_1110, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.scfsi(), 8);
    }

    #[test]
    fn test_scfsi_stereo() {
        let data = vec![0b_1111_1111, 0b_1111_0000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.scfsi(), 0);

        let data = vec![0b_1111_1111, 0b_1111_0000, 0b_0001_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.scfsi(), 1);

        let data = vec![0b_1111_1111, 0b_1111_1000, 0b_0000_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.scfsi(), 128);

        let data = vec![0b_1111_1111, 0b_1111_0100, 0b_0001_0000, 0b_0000_0000];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.scfsi(), 65);
    }

    #[test]
    fn test_part2_3_length_mono() {
        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1100_0000, //2
            0b_0000_0000, //3
            0b_0000_0000, //4
            0b_0000_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.part2_3_length(), 0);

        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1100_0000, //2
            0b_0000_0100, //3
            0b_0000_0000, //4
            0b_0000_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.part2_3_length(), 1);

        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1110_0000, //2
            0b_0000_0000, //3
            0b_0000_0000, //4
            0b_0000_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, true);
        assert_eq!(body.part2_3_length(), 2048);
    }
    #[test]
    fn test_part2_3_length_stereo() {
        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1111_0000, //2
            0b_0000_0000, //3
            0b_0000_0000, //4
            0b_0000_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.part2_3_length(), 0);

        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1111_0000, //2
            0b_0000_0000, //3
            0b_0000_0000, //4
            0b_0001_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.part2_3_length(), 1);

        let data = vec![
            0b_1111_1111, //0
            0b_1111_1111, //1
            0b_1111_1000, //2
            0b_0000_0000, //3
            0b_0000_0000, //4
            0b_0000_0000, //5
            0b_0000_0000, //6
            0b_0000_0000, //7
        ];
        let body = Mp3Body::new(data, false);
        assert_eq!(body.part2_3_length(), 8388608);
    }
}
