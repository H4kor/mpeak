// References
// https://www.diva-portal.org/smash/get/diva2:830195/FULLTEXT01.pdf

pub struct Mp3Body {
    pub data: Vec<u8>,
}

impl Mp3Body {
    pub fn new(data: Vec<u8>) -> Mp3Body {
        Mp3Body { data: data }
    }
}
