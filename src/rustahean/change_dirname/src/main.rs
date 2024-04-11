use std::fs;
use std::env;
use std::path::Path;

/*
OOO_0, OOO_1, ... みたいなディレクトリがあって、OOOをXXXに変えたいときに使える。
（runvasp.pyで作ったデータディレクトリの名前を一括で変更するために作りました。）
第一引数にディレクトリが入っているpath,  required
第二引数に古いデータディレクトリの名前,  required
第三引数に新しいデータディレクトリの名前, required
第四引数に数字をいくつずらすか, optional
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 4 || args.len() == 5);
    let mut dir_path_str: &str = &args[1];
    if dir_path_str.ends_with('/') {
        dir_path_str = dir_path_str.strip_suffix('/').unwrap_or(dir_path_str);
    }
    let old_name: &str = &args[2];
    let new_name: &str = &args[3];
    let shift_num_size: i32 = if args.len() == 4 { 0 } else { (&args[4]).parse().unwrap() };

    search_files(dir_path_str, old_name, new_name, &shift_num_size);
}

fn search_files(dir_path_str: &str, old_name: &str, new_name: &str, shift_num_size: &i32) {
    let dirs = fs::read_dir(dir_path_str).unwrap();
    for dir in dirs {
        let path = dir.unwrap();
        if let Some((before_last, last)) = path.path().to_str().unwrap().rsplit_once('_') {
            if before_last == format!("{}/{}", dir_path_str, old_name) {
                if let Ok(idx) = last.parse::<i32>() {
                    let new_path = Path::new(dir_path_str).join(format!("{}_{}", new_name, idx + shift_num_size));
                    let old_path = Path::new(dir_path_str).join(format!("{}_{}", old_name, idx));
                    if let Err(err) = fs::rename(old_path.to_str().unwrap(), new_path.to_str().unwrap()) {
                        println!("Error: {}", err);
                    } else {
                        println!("Changed {} -> {}", format!("{}_{}", old_name, idx), format!("{}_{}", new_name, idx + shift_num_size));
                    }       
                }         
            }
        }
    }
}
