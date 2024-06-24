/*
ovitoでのDXAで出力したステップごとの転位長さファイルを
pd.read_csv(
        path, sep='\s+', header=0, dtype=np.float64
    )
で読み込める形に出力する。

第1引数 : stepごとのファイルが入ったディレクトリのパス
第2引数 : stepごとのファイルの名前 dislocation.0 みたいにファイルがあるなら dislocation と指定
第3引数 : 出力される新しいファイルの名前（第1引数/第3引数)の場所に作られる。
*/

use std::fs;
use std::env;
use std::io::{BufReader, BufWriter, BufRead, Write};

fn main() {
    let a = Args::new(&env::args().collect());
    let dislocation_vec: Vec<(usize, Dislocation)> = get_dislocation_data(&a);
    csv_out(&a, &dislocation_vec);
}

fn csv_out(a: &Args, vec: &Vec<(usize, Dislocation)>){
    let mut f = BufWriter::new(fs::File::create(&a.out).unwrap());
    writeln!(f, "Step Perfect Shockkey Stair-rod Hirth Frank Sum").unwrap();
    for (step, dislocation) in vec.iter() {
        writeln!(f, "{} {} {} {} {} {} {}",
        step,
        dislocation.perfect,
        dislocation.shockkey,
        dislocation.stair_rod,
        dislocation.hirth,
        dislocation.frank,
        dislocation.sum).unwrap();
    }
}

fn get_dislocation_data(a: &Args) -> Vec<(usize, Dislocation)> {
    let mut dislocation_vec: Vec<(usize, Dislocation)> = vec![];
    let entries = fs::read_dir(&a.dir).unwrap();
    for entry in entries {
        let path = entry.unwrap();
        if let Some(file_name) = path.path().file_name().unwrap().to_str() {
            if file_name.starts_with(&a.name) {
                let step: usize = file_name.split('.')
                .nth(1).unwrap()
                .parse::<usize>().unwrap();
                let d: Dislocation = extract_dislocation(path.path().to_str().unwrap());
                dislocation_vec.push((step, d));
            }
        }
    } 

    dislocation_vec.sort_by(|a, b| a.0.cmp(&b.0));
    dislocation_vec
}

fn extract_dislocation(path: &str) -> Dislocation {
    let mut d = Dislocation::default();
    let f = fs::File::open(path).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.expect("REASON");
        let spline: Vec<&str> = line.split_whitespace().collect();
        if spline.len() < 2 {
            continue;
        }
        match spline[0] {
            "Other" => {
                d.other = spline[1].parse::<f64>().unwrap();
            }
            "\"1/2<110>" => {
                d.perfect = spline[2].parse::<f64>().unwrap();
            }
            "\"1/6<112>" => {
                d.shockkey = spline[2].parse::<f64>().unwrap();
            }
            "\"1/6<110>" => {
                d.stair_rod = spline[2].parse::<f64>().unwrap();
            }
            "\"1/3<100>" => {
                d.hirth = spline[2].parse::<f64>().unwrap();
            }
            "\"1/3<111>" => {
                d.frank = spline[2].parse::<f64>().unwrap();
            }
            _ => { continue; }
        }
    }
    d.summize();
    d
}

#[derive(Default)]
struct Dislocation {
    other: f64,
    perfect: f64,
    shockkey: f64,
    stair_rod: f64,
    hirth: f64,
    frank: f64,
    sum: f64,
}

impl Dislocation {
    fn summize(&mut self) {
        self.sum = self.other + self.perfect + self.shockkey + self.stair_rod + self.hirth + self.frank; 
    }
}

#[derive(Debug)]
struct Args {
    dir: String,
    name: String,
    out: String,
}

impl Args {
    fn new(a: &Vec<String>) -> Self {
        assert!(a.len() == 4);
        Args {dir: a[1].clone(), name: a[2].clone(), out: a[1].clone() + &a[3].clone()}
    }
}
