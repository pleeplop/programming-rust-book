use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::io::{BufRead, ErrorKind, stderr, Write};
use std::path::Path;

fn main() {
    let share = pirate_share(345, 23);
    println!("pirate_share {}", share);

    let mut map: HashMap<String, char> = HashMap::new();
    let key1 = String::from("key1");
    map.insert(key1.clone(), 'e');
    println!("entry [{},{}]", key1, map.get(&key1).unwrap());

    result();

    if let Err(err) = get_weather("toulouse") {
        print_error(&err);
        std::process::exit(1);
    }
}

#[derive(Debug)]
enum WeatherReport {
    Sunny(i32),
    Cloudy,
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}

fn caused_of<'a>(mut err: &'a (Error + 'static)) -> Option<&'a io::Error> {
    loop {
        match err.downcast_ref::<io::Error>() {
            None => match err.source() {
                None => return None,
                Some(cause) => err = cause,
            },
            d => return d,
        }
    }
}

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn read_numbers(file: &mut BufRead) -> GenResult<Vec<i64>> {
    let mut v = vec![];
    for line in file.lines() {
        let line = line?;
        v.push(line.parse()?);
    }
    Ok(v)
}

type IoResult<T> = Result<T, io::Error>;

fn result() {
    const THE_USUAL: WeatherReport = WeatherReport::Sunny(72);
    let toulouse = "toulouse";
    let report = get_weather(toulouse).unwrap_or(THE_USUAL);
    display_weather(toulouse, &report);

    let report = Err(io::ErrorKind::NotFound).unwrap_or_else(|_e| vague_prediction(toulouse));
    display_weather(toulouse, &report);
}

fn get_weather(_location: &str) -> IoResult<WeatherReport> {
    Result::Ok(WeatherReport::Sunny(72))
}

fn display_weather(location: &str, report: &WeatherReport) {
    println!("{} weather: {:?}", location, report);
}

fn vague_prediction(_location: &str) -> WeatherReport {
    WeatherReport::Cloudy
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

pub struct LipogramCorpora {
    selections: Vec<(char, Option<String>)>,
}

impl LipogramCorpora {
    pub fn validate_all(&mut self) -> Result<(), char> {
        for selection in &self.selections {
            if let Some(text) = selection.1.as_ref() {
                if text.contains(selection.0) {
                    return Err(selection.0);
                }
            }
        }
        Ok(())
    }
}

struct AThing<'a> {
    pub name: &'a str,
}

impl<'a> AThing<'a> {
    pub fn thing_len(&self) -> usize {
        self.name.len()
    }
}

fn clone_or_not_clone() {
    let dj_name = String::from("don juan");
    let my_thing = AThing { name: &dj_name };

    // static lifetime
//    let dj_name = "don juan";
//    let my_thing = AThing { name: dj_name };

    println!("{} is {} chars long", my_thing.name, my_thing.thing_len());
}

fn print_error(error: &Error) {
    writeln!(stderr(), "err: {}", error.description());
    let mut current = error.source();
    while let Some(source) = current {
        writeln!(stderr(), "caused by: {}", source.description());
        current = source.source();
    }
}
