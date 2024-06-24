use std::fs;
use std::env;
use std::io::{BufReader, BufWriter, BufRead, Write};

/*
ovitoの Polyhedral template matching で出力したステップごとの相割合を
pd.read_csv(
        path, sep='\s+', header=0, dtype=np.int64
    )
で読み込める形に出力する。

第1引数 : stepごとのファイルが入ったディレクトリのパス
第2引数 : stepごとのファイルの名前 phase_ratio.0 みたいにファイルがあるなら dislocation と指定
第3引数 : 出力される新しいファイルの名前（第1引数/第3引数)の場所に作られる。
*/

fn main() {
    let a = Args::new(&env::args().collect());
    let phase_vec: Vec<(usize, Phase)> = get_phase_ratio_data(&a);
    csv_out(&a, &phase_vec);
}

fn csv_out(a: &Args, vec: &Vec<(usize, Phase)>){
    let mut f = BufWriter::new(fs::File::create(&a.output).unwrap());
    writeln!(f, "Step Other FCC HCP BCC").unwrap();
    for (step, phase) in vec.iter() {
        writeln!(f, "{} {} {} {} {}",
        step,
        phase.other,
        phase.fcc,
        phase.hcp,
        phase.bcc).unwrap();
    }
}

fn get_phase_ratio_data(a: &Args) -> Vec<(usize, Phase)> {
    let mut phase_vec: Vec<(usize, Phase)> = vec![];
    let entries = fs::read_dir(&a.dir).unwrap();
    for entry in entries {
        let path = entry.unwrap();
        if let Some(file_name) = path.path().file_name().unwrap().to_str() {
            if file_name.starts_with(&a.name) {
                let step: usize = file_name.split('.')
                .nth(1).unwrap()
                .parse::<usize>().unwrap();
                let p: Phase = extract_phase(path.path().to_str().unwrap());
                phase_vec.push((step, p));
            }
        }
    } 

    phase_vec.sort_by(|a, b| a.0.cmp(&b.0));
    phase_vec
}

fn extract_phase(path: &str) -> Phase {
    let mut p = Phase::default();
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
                p.other = spline[1].parse::<i64>().unwrap();
            }
            "FCC" => {
                p.fcc = spline[1].parse::<i64>().unwrap();
            }
            "HCP" => {
                p.hcp = spline[1].parse::<i64>().unwrap();
            }
            "BCC" => {
                p.bcc = spline[1].parse::<i64>().unwrap();
            }
            _ => { continue; }
        }
    }
    p
}

#[derive(Default)]
struct Phase {
    other: i64,
    fcc: i64,
    hcp: i64,
    bcc: i64,
}

#[derive(Debug)]
struct Args {
    dir: String,
    name : String,
    output : String,
}

impl Args {
    fn new(a: &Vec<String>) -> Self {
        assert!(a.len() == 4);
        Args {dir: a[1].clone(), name: a[2].clone(), output: a[1].clone() + &a[3].clone()}
    }
}