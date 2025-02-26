use std::collections::VecDeque;
use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader,Read};
use std::process::exit;
use regex::Regex;
use console::{style, Emoji};
use clearscreen;
use kdam::{term, tqdm, BarExt, Spinner};
use bat::{PagingMode, PrettyPrinter, WrappingMode};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};

extern crate fstream;

// STATIC VARS
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static STAR: Emoji<'_, '_> = Emoji("✨", "");
static NUMBER: Emoji<'_, '_> = Emoji("📱", "");
static FILE: Emoji<'_, '_> = Emoji("📄", "");
static  WASTE: Emoji<'_,'_> = Emoji("🗑️","");
// ---

const BANNER: &str = r"
   _____                      __    ____      __
  / ___/___  ____ ___________/ /_  /  _/___  / /___  ____ _
  \__ \/ _ \/ __ `/ ___/ ___/ __ \ / // __ \/ / __ \/ __ `/
 ___/ /  __/ /_/ / /  / /__/ / / // // / / / / /_/ / /_/ /
/____/\___/\__,_/_/   \___/_/ /_/___/_/ /_/_/\____/\__, /
                                                  /____/
";

fn get_file_row(file: &str) -> u64 {
    let file = BufReader::new(File::open(file).expect("Unable to open file"));
    let mut cnt = 0;
    for _ in file.lines() {
        cnt = cnt + 1;
    }
    cnt
}
// fn get_line_at(path: &Path, line_num: usize) -> Result<String, std::io::Error> {
//     let file = File::open(path).expect("File non trovato o impossibile da aprire!");
//     let content = BufReader::new(&file);
//     let mut lines = content.lines();
//     lines.nth(line_num).expect("Linea non trovata alla posizione specificata!")
// }

fn search(filename: &str, search_line: &str, n_file: u64) -> Result<String, std::io::Error> {
    let n_row = get_file_row(filename);
    let file = match File::open(filename){
        Ok(f) => f,
        Err(_e) =>
            {
                let mut file = File::open(filename)?;

                // Read the file into a byte vector
                let mut byte_buffer = Vec::new();
                file.read_to_end(&mut byte_buffer)?;

                // Decode the bytes using the desired encoding (_e.g., ISO-8859-1)
                let decoded_string = ISO_8859_1.decode(&byte_buffer, DecoderTrap::Strict).unwrap();
                return Ok(decoded_string);
            }
    };

    let file_res = format!("search_result_{}.log", n_file);
    let mut reader = BufReader::with_capacity(2048 * 2048, file);
    let mut line_numbers = VecDeque::new();
    let mut line_number = 0;
    let numero_file = format!("[{}]", n_file);

    println!("{} {} Elaboro file => {} N.Righe: {} ", style(numero_file).bold().dim(), FILE, filename, n_row);
    let mut pb = tqdm!(
        total = n_row as usize ,
        force_refresh = true,
        bar_format = "{desc suffix=' '}|{animation}| {spinner} {count}/{total} [{percentage:.0}%] in {elapsed human=true} ({rate:.1}/s, eta: {remaining human=true})",
        spinner = Spinner::new(
            &["▁▂▃", "▂▃▄", "▃▄▅", "▄▅▆", "▅▆▇", "▆▇█", "▇█▇", "█▇▆", "▇▆▅", "▆▅▄", "▅▄▃", "▄▃▂", "▃▂▁"],
            30.0,
            1.0,
        )
    );
    let mut txt_log: String = format!("NomeFile: {}\r\n", filename);
    loop {
        let mut line = String::new();
        line_number += 1;
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let reg_exp = format!(r"(?xi)(\b{}\b)", search_line);
        let regex_where = Regex::new(&*reg_exp).unwrap();
        let result_where = regex_where.captures_iter(&line).count();

        if result_where > 0 {
            line_numbers.push_back(line_number);
            println!("{} {} Trovato in {} numero linea {}", style("[=>]").bold().dim(), FILE, filename, line_number);
            let new_line = format!("{} {}", line, "\r\n");
            txt_log += &*new_line;
        }
        pb.update(1).unwrap();
    }
    if line_numbers.is_empty() {
        println!("La ricerca non ha fornito risultati.");
    } else {
        fstream::write_text(file_res, txt_log, true);
    }
    pb.set_bar_format("{desc suffix=' '}|{animation}| {count}/{total} [{percentage:.0}%] in {elapsed human=true} ({rate:.1}/s)").unwrap();
    pb.clear().unwrap();
    pb.refresh().unwrap();
    Ok("".to_string())
}

fn get_res_files(path: String) -> Vec<String> {
    let mut fvec: Vec<String> = Vec::new();
    let dir = fs::read_dir(path).unwrap(); //std::path::Path::new(&path).read_dir().unwrap();
    for element in dir {
        let pf = element.unwrap().path();
        if pf.is_file() {
            let nf = pf.display().to_string();
            if nf.contains(".log") {
                fvec.push(nf);
            }
        }
    }
    fvec
}

fn main() {
    clearscreen::clear().unwrap();
    term::init(false);
    term::hide_cursor().unwrap();
    let mut file_name;
    let mut search_text;
    let args: Vec<String> = env::args().collect();
    println!("{}",BANNER);
    println!("search-in-log v{} by #GR (C) 2024 CippoLippo Enterprise ", env!("CARGO_PKG_VERSION"));
    println!();

    if args.len() == 1 {
        println!("Nessun paramentro specificato");
        exit(1);
    }
    println!("{} {} Pulizia ricerca precedente:", style("[*]").bold().dim(), WASTE);
    let delete_log = get_res_files(env::current_dir().unwrap().display().to_string());
    for dl in tqdm!(delete_log.iter()){
        fs::remove_file(dl).expect("Errore nella cancellazione file log!");
    }

    let p_files = fs::read_dir(args[1].to_string()).unwrap();
    let n_file = fs::read_dir(args[1].to_string()).unwrap().count() as u64;

    println!("{} {} Percorso di ricerca: {}", style("[*]").bold().dim(), TRUCK, args[1].to_string());
    println!("{} {} String di ricerca: {}", style("[*]").bold().dim(), STAR, args[2].to_string());
    println!("{} {} Numero Files: {}", style("[*]").bold().dim(), NUMBER, n_file);

    let mut i = 0;
    for f in p_files {
        let file = f;
        let path = file.unwrap().path();
        if path.is_file()
        {
            i += 1;
            search_text = args[2].to_string();
            file_name = path.display().to_string();
            let curr_file = file_name.clone();
            _ =  match search(&{ file_name }, &{ search_text }, i){
                Ok(_) => println!("Fine Ricerca nel file {}",curr_file),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    let curr_path = env::current_dir().unwrap().display().to_string();
    let result_log = get_res_files(curr_path);
    loop {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Seleziona:")
            .default(0)
            .items(&result_log)
            .interact()
            .unwrap();

        PrettyPrinter::new()
            .input_file(&result_log[selection])
            .language("json")
            .line_numbers(true)
            .grid(true)
            .header(true)
            .wrapping_mode(WrappingMode::Character)
            .paging_mode(PagingMode::QuitIfOneScreen)
            .print()
            .unwrap();
    }

}

