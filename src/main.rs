
use std::fs;
use std::env;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <src_dir> <out_dir>", args[0]);
        std::process::exit(1);
    }

    let src_dir = &args[1];
    let out_dir = &args[2];

    // Collect all entries first for parallel processing
    let entries: Vec<_> = match fs::read_dir(src_dir) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(e) => {
            eprintln!("Failed to read src_dir: {}", e);
            std::process::exit(1);
        }
    };

    let files_per_dir = 100;
    let total_files = entries.len();
    let total_dirs = (total_files + files_per_dir - 1) / files_per_dir;

    // Pre-create all output directories
    (1..=total_dirs).into_par_iter().for_each(|dircount| {
        let dir_path = format!("{}/{}", out_dir, dircount);
        if let Err(e) = fs::create_dir_all(&dir_path) {
            eprintln!("Unable to create directory {}: {}", dir_path, e);
        }
    });

    // Prepare file copy jobs
    let jobs: Vec<_> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let dircount = (i / files_per_dir) + 1;
            let path = entry.path();
            let filename = match path.file_name().and_then(|f| f.to_str()) {
                Some(f) => f,
                None => "",
            };
            let fname = filename.rsplit('/').next().unwrap_or("");
            let in_str = format!("{}/{}", src_dir, filename);
            let out_dir_path = format!("{}/{}", out_dir, dircount);
            let out_str = format!("{}/{}", out_dir_path, fname);
            (in_str, out_str, fname.to_string())
        })
        .collect();

    // Parallel file copy
    jobs.par_iter().for_each(|(in_str, out_str, fname)| {
        println!("\nthis one: \n{} ->\n{}\n", in_str, out_str);
        match fs::copy(in_str, out_str) {
            Ok(_) => println!("File copied successfully"),
            Err(e) => eprintln!("Error copying file {}: {}", fname, e),
        }
    });
}
