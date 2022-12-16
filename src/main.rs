use std::error::Error;

use clap::Parser;
use file_size::fit_4;
use walkdir::WalkDir;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

const DIR: &str = "./";
const NUM: usize = 10;

struct Entry {
    path: String,
    size: u64,
}

fn main() {
    match get_dir_size(DIR) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_dir_size(dir: &str) -> Result<(), Box<dyn Error>> {
    let mut size: u64 = 0;
    let mut count: u64 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(NUM + 1);
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
                tops.truncate(NUM);
                min_tops = tops.last().unwrap().size;
            }
            size += path.metadata().unwrap().len();
            count += 1;
        }
    }

    println!("{} files, {} bytes", count, fit_4(size));
    println!("{} largest files:", NUM);
    println!("{} | {}", "Size", "Path");
    for t in tops {
        println!("{} | {}", fit_4(t.size), t.path);
    }

    Ok(())
}
