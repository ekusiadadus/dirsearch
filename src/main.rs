use std::error::Error;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    number: usize,
}

use file_size::fit_4;
use walkdir::WalkDir;

const DIR: &str = "./";
const NUM: usize = 10;

fn main() {
    let cli = Cli::parse();
    let num = cli.number;
    let dir = DIR;

    if num == 0 {
        println!("Number of files to show must be greater than 0");
        return;
    }

    get_dir_size(dir, num).unwrap();
}

struct Entry {
    path: String,
    size: u64,
}

fn get_dir_size(dir: &str, num: usize) -> Result<(), Box<dyn Error>> {
    let mut size: u64 = 0;
    let mut count: u64 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(num + 1);
    let mut min_tops: u64 = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let t = path.metadata().unwrap().len();
            if t > min_tops {
                tops.push(Entry {
                    path: path.to_str().unwrap().to_string(),
                    size: t,
                });
                tops.sort_by(|a, b| b.size.cmp(&a.size));
                tops.truncate(num);
                min_tops = tops.last().unwrap().size;
            }
            size += path.metadata().unwrap().len();
            count += 1;
        }
    }

    println!("{} files, {} bytes", count, fit_4(size));
    println!("{} largest files:", num);
    println!("{} | {}", "Size", "Path");
    for t in tops {
        println!("{} | {}", fit_4(t.size), t.path);
    }

    Ok(())
}
