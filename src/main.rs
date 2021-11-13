use std::env;

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
            let headers = mpeak::all_headers(&file_data);
            println!("frames              {:?}", headers.len());
            println!("header              {:?}", headers[0]);
            println!("header frame_length {:?}", headers[0].frame_length());
        }
        Err(e) => println!("{:?}", e),
    };
}
