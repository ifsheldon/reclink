use clap::{App, Arg};
use std::path::PathBuf;
use glob::glob;
use std::process::exit;

// FIXME: due to glob() limitations, example like """cargo run "target" "test_dir" -i="debug/build/**/*" """ will include directory "target/build" which is a minor issue

fn main() {
    let matches = App::new("Reclink")
        .arg(Arg::new("source_dir").required(true))
        .arg(Arg::new("dst_dir").required(true))
        .arg(Arg::new("ignore_patterns")
            .long("ignore_patterns")
            .short('i')
            .takes_value(true)
            .multiple_occurrences(true))
        .get_matches();
    let source_dir = matches.value_of("source_dir").unwrap();
    let dst_dir = matches.value_of("dst_dir").unwrap();
    let source_dir_path = PathBuf::from(source_dir);
    let dst_dir_path = PathBuf::from(dst_dir);
    if !dst_dir_path.exists() {
        eprintln!("destination directory not exist, created");
        std::fs::create_dir(dst_dir_path.clone()).expect("Error when creating destination directory");
    }
    let ignore_paths = match matches.values_of("ignore_patterns") {
        None => Vec::new(),
        Some(pattens) => {
            let ignore_paths = pattens.into_iter().flat_map(
                |ignore_patten| {
                    let ignore_source_patten = source_dir_path.join(ignore_patten);
                    let ignore_paths = glob(ignore_source_patten.to_str().unwrap())
                        .unwrap()
                        .collect::<Vec<Result<_, _>>>()
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap();
                    ignore_paths
                }
            ).collect::<Vec<PathBuf>>();
            ignore_paths
        }
    };
    let all_paths = glob(source_dir_path.join("**/*").to_str().unwrap())
        .unwrap()
        .collect::<Vec<Result<_, _>>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut all_valid_paths = all_paths.into_iter()
        .filter(|path| !ignore_paths.contains(path))
        .collect::<Vec<_>>();
    all_valid_paths.sort();
    let new_path_pairs = all_valid_paths.into_iter()
        .map(|path| {
            let sub_path = path.strip_prefix(source_dir_path.clone()).expect("Something wrong with stripping");
            let new_path = dst_dir_path.join(sub_path);
            (path, new_path)
        }).collect::<Vec<_>>();
    let existed_new_path = new_path_pairs.iter()
        .filter(|(_, new_path)| new_path.exists())
        .collect::<Vec<_>>();
    if !existed_new_path.is_empty() {
        eprintln!("Some files exists! Existed files are:");
        existed_new_path.iter().for_each(|(_, new_path)| eprintln!("{}", new_path.to_str().expect("Another error occurred when printing existed file")));
        exit(-1);
    }
    new_path_pairs.into_iter()
        .for_each(|(old_path, new_path)| {
            let new_path_str = String::from(new_path.to_str().unwrap());
            let old_path_str = String::from(old_path.to_str().unwrap());
            if old_path.is_dir() {
                println!("{} is a directory <-> {}", old_path_str, new_path_str);
                std::fs::create_dir(new_path).expect(format!("Error when creating dir {}", new_path_str).as_str());
            } else {
                // hard link file
                println!("{} is a file <-> {}", old_path_str, new_path_str);
                std::fs::hard_link(old_path, new_path).expect(format!("Error when creating hard link from {} to {}", old_path_str, new_path_str).as_str());
            }
        });
}
