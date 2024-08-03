use std::collections::VecDeque;
use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::{Regex};
use console::{style, Emoji};
use clearscreen;
use std::thread;
// STATIC VARS
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static STAR: Emoji<'_, '_> = Emoji("✨", "");
static NUMBER: Emoji<'_, '_> = Emoji("📱", "");
static FILE: Emoji<'_, '_> = Emoji("📄", "");
// ---

fn get_file_row(file: &str) -> u64 {
    let file = BufReader::new(File::open(file).expect("Unable to open file"));
    let mut cnt = 0;
    for _ in file.lines() {
        cnt = cnt + 1;
    }
    return cnt;
}

fn main() {
    clearscreen::clear().unwrap();
    println!("search-in-log v0.1 by #GR (C) 2024 CippoLippo Enterprise ");
    println!();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Nessun paramentro specificato");
        exit(1);
    }
    let p_files = fs::read_dir(args[1].to_string()).unwrap();
    let n_file = fs::read_dir(args[1].to_string()).unwrap().count() as u64;

    println!("{} {} Percorso di ricerca: {}", style("[*]").bold().dim(), TRUCK, args[1].to_string());
    println!("{} {} String di ricerca: {}", style("[*]").bold().dim(), STAR, args[2].to_string());
    println!("{} {} Numero Files: {}", style("[*]").bold().dim(), NUMBER, n_file);
    let mut file_name;
    let mut search_text;
    let mut x = 0;
    for f in p_files {
        x += 1;
        search_text = args[2].to_string();
        file_name = f.unwrap().path().display().to_string();
        let h = thread::spawn(move || {
            let _ = search(&{ file_name }, &{ search_text });
        });
        if x == 5 {
            x = 0;
            println!("Attendo filne elaborazione 5 file.");
            h.join().unwrap();
        }
    }
}

fn search(filename: &str, search_line: &str) -> Result<VecDeque<u32>, std::io::Error> {
    let n_row = get_file_row(filename);
    let file = File::open(filename)?;
    let mut reader = BufReader::with_capacity(10000000, file);  //BufReader::with_capacity(2048 * 2048, file);
    let mut line_numbers = VecDeque::new();
    let mut line_number = 0;
    let start = std::time::Instant::now();
    let mut line = String::new();
    println!("{} {} Elaboro file => {} N.Righe: {} ", style("[*]").bold().dim(), FILE, filename, n_row);
    loop {
        line_number += 1;
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let reg_exp = format!(r"(?xi)(\b{}\b)", search_line);
        let regex_where = Regex::new(&*reg_exp).unwrap();
        let result_where = regex_where.captures_iter(&line.trim()).count();
        if result_where > 0 {
            line_numbers.push_back(line_number);
            println!("Trovato in {} numero linea {}\r", filename, line_number);
            break;
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    if line_numbers.is_empty() {
        println!("La ricerca non ha fornito risultati.");
    }
    Ok(Default::default())
}
