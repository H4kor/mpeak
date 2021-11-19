use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;

fn mix() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = env::args().collect();
    let file_path1 = &args[1];
    let file_path2 = &args[2];
    let file1 = mpeak::load_file(file_path1).unwrap();
    let file2 = mpeak::load_file(file_path2).unwrap();
    let frames1 = mpeak::get_frames(&file1).unwrap();
    let frames2 = mpeak::get_frames(&file2).unwrap();

    let mut n_frames = Vec::<u8>::new();
    for i in 0..usize::min(frames1.len(), frames2.len()) {
        let p = rng.gen::<f64>();
        if p < 0.5 {
            n_frames.extend(&frames1[i].body.data);
        } else {
            n_frames.extend(&frames2[i].body.data);
        }
    }

    let mut file = File::create("output.mp3").unwrap();
    file.write_all(n_frames.as_slice()).unwrap();
    file.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_result = mpeak::load_file(file_path);
    match file_result {
        Ok(file_data) => {
            println!("is mp3:             {:?}", mpeak::is_mp3_file(&file_data));
            println!("has id3:            {:?}", mpeak::has_id3(&file_data));
            println!(
                "id3 offset:         {:?}",
                mpeak::get_id3_offset(&file_data)
            );
            println!(
                "id3 data:           {:?}",
                String::from_utf8_lossy(&mpeak::get_id3_data(&file_data))
            );
            let frames = mpeak::get_frames(&file_data).unwrap();
            for frame in frames {
                println!("{:?}", frame.header.frame_length());
            }
            // println!("frames              {:?}", frames.len());
            // println!("header              {:?}", frames[0].header);
            // println!("header frame_length {:?}", frames[0].header.frame_length());
        }
        Err(e) => println!("{:?}", e),
    };
}
