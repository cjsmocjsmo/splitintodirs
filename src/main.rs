use std::fs::{create_dir, read_dir};

fn main() {
    let src_dir = "/media/pipi/USB01/DeDuped1/";
    let base_dir = "/media/pipi/USB01/Master/";
    let dest_dir_a = base_dir.to_owned() + "001";
    let dest_dir_b = base_dir.to_owned() + "002";
    let dest_dir_c = base_dir.to_owned() + "003";
    let dest_dir_d = base_dir.to_owned() + "004";
    let dest_dir_e = base_dir.to_owned() + "005";
    let dest_dir_f = base_dir.to_owned() + "006";
    let dest_dir_g = base_dir.to_owned() + "007";
    let dest_dir_h = base_dir.to_owned() + "008";
    let dest_dir_i = base_dir.to_owned() + "009";
    let dest_dir_j = base_dir.to_owned() + "010";

    let mut file_count = 0;

    let mut dest_dir = dest_dir_a.clone();

    for dir_entry in read_dir(src_dir).unwrap() {
        if let Ok(dir_entry) = dir_entry {
            let file_name = dir_entry.file_name();
            // let file_name = dir_entry.file_name().unwrap().to_str().unwrap();

            if file_count == 18000 {
                dest_dir = dest_dir_j.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 16000 {
                dest_dir = dest_dir_i.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 14000 {
                dest_dir = dest_dir_h.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 12000 {
                dest_dir = dest_dir_g.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 10000 {
                dest_dir = dest_dir_f.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 8000 {
                dest_dir = dest_dir_e.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 6000 {
                dest_dir = dest_dir_d.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 4000 {
                dest_dir = dest_dir_c.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 2000 {
                dest_dir = dest_dir_b.clone();
                create_dir(dest_dir.clone()).unwrap();
            } else if file_count == 0 {
                dest_dir = dest_dir_a.clone();
                create_dir(dest_dir.clone()).unwrap();
            }

            let src_path = std::path::Path::new(src_dir).join(file_name.clone());
            let dst_path = std::path::Path::new(&dest_dir).join(file_name.clone());

            // Create a longer lived value for the destination path
            let dst_path = dst_path.clone();

            // Move the file to the destination directory
            std::fs::rename(src_path, dst_path).unwrap();
            println!("Moved file {:?} to {:?}", file_name, dest_dir);

            file_count += 1;
        }
    }
}
