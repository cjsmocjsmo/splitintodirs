use std::fs;
use std::fs::read_dir;

fn main() {
    // let src_dir = "/media/pi/taz/Master_Master_Resize2";
    // // loop throught src_dir collecting filenames into a vec
    // let mut grand_vec = Vec::new();
    // let mut count = 0;
    // for entry in read_dir(src_dir).unwrap() {
    //     let mut fn_vec = Vec::new();
    //     count += 1;
    //     if count < 101 {
    //         let entry = entry.unwrap();
    //         let path = entry.path();
    //         let filename = path.file_name().unwrap().to_str().unwrap();
    //         fn_vec.push(filename.to_string());

    //     } else {
    //         count = 0;
    //         grand_vec.push(fn_vec);
    //     }

    // }

    // print!("{:?}", grand_vec);

    let src_dir = "/media/pipi/0123-4567/JPG";
    let out_dir = "/media/pipi/0123-4567/Master_Split";
    // loop throught src_dir collecting filenames into a vec
    // let mut fn_vec: Vec<String> = Vec::new();
    let mut dircount = 1;
    let mut count = 0;
    for entry in read_dir(src_dir).unwrap() {
        count += 1;
        if count < 101 {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let fn_split = filename.split("/").collect::<Vec<&str>>();
            let fname = fn_split.last().unwrap().to_string();
            let in_str = format!("{}{}", src_dir, filename);
            let out_dir = format!("{}/{}", out_dir, dircount);
            //create our_dir if it doesn't exist
            if !fs::metadata(&out_dir).is_ok() {
                fs::create_dir(&out_dir).expect("Unable to create directory");
            }
            let out_str = format!("{}/{}", out_dir, fname);
            let res = format!("{} ->\n{}", in_str, out_str);
            print!("{}", res);
            fs::copy(in_str, out_str).expect("Unable to copy file");
        } else {
            count = 0;
            dircount += 1;
        }
    }
}
