use std::fs;
use std::fs::read_dir;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <src_dir> <out_dir>", args[0]);
        std::process::exit(1);
    }

    let src_dir = &args[1];
    let out_dir = &args[2];
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
            let in_str = format!("{}/{}", src_dir, filename);
            let out_dir = format!("{}/{}", out_dir, dircount);

            // Create out_dir if it doesn't exist
            if !fs::metadata(&out_dir).is_ok() {
                fs::create_dir_all(&out_dir).expect("Unable to create directory");
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
