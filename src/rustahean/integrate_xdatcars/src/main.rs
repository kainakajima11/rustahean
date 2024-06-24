/*
runvaspを実行したディレクトリのパスをargsで与える
その下にあるXDATCARを連結して,vasp_dirにXDATCARとして出力
 */

extern crate glob;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

 fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let binding = std::fs::canonicalize(&args[1]).unwrap();
    let vasp_dir = binding.to_str().unwrap();

    let xdat_pattern = format!("{}/**/XDATCAR", vasp_dir);

    let xdatcars = match glob(&xdat_pattern) {
        Ok(pattern) => pattern,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, format!("glob error: {}", err)));
        }
    };

    let output_path = format!("{}/XDATCAR", vasp_dir);
    let mut output_file = File::create(output_path)?;

    for xdatcar_path in xdatcars {
        let mut contents = String::new();
        let xdatcar_path = match xdatcar_path {
            Ok(path) => path,
            Err(err) => {
                return Err(io::Error::new(io::ErrorKind::Other, format!("glob error: {}", err)));
            }
        };
        let mut f_xdat = File::open(&xdatcar_path)?;
        f_xdat.read_to_string(&mut contents)?;
        output_file.write_all(contents.as_bytes())?;
    }

    Ok(())
}