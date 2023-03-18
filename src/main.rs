use crate::setting::Config;
use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

mod setting;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let filepath = "ping_output.txt";

    let config: Config = load_setting();

    if args.len() > 1 {
        let command: String = args[1].parse().unwrap();
        run_program(command, config)
    }
}

fn run_program(command: String, config: Config) {
    match command.as_str() {
        code if code == &config.command_run => run_speed_test(config),
        code if code == &config.command_show => calculate_file_speed_test(&config.filepath),
        code if code == &config.command_cls => match clear_file_txt(&config.filepath) {
            Ok(_) => println!("File cleared successfully"),
            Err(e) => eprint!("Error clearing file: {}", e),
        },
        _ => println!("Error: Command '{}' not exist", command),
    }
}

fn load_setting() -> Config {
    let config: Config = match fs::read_to_string("setting.json") {
        Ok(config_str) => serde_json::from_str(&config_str),
        Err(_) => panic!("Failed to read config file"),
    }
    .unwrap();

    config
}

fn get_data_speedtest(input: String) -> Vec<String> {
    let re = Regex::new(r"(Latency|Download|Upload):\s+(\d+(?:\.\d+)?)\s+(\w+)").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(&input) {
        let metric = cap[1].to_string();
        let value = cap[2].to_string();
        let unit = cap[3].to_string();
        let result = format!("{}: {} {}", metric, value, unit);
        results.push(result);
    }
    results
}

fn run_speed_test(config: Config) {
    let mut counter = 0;

    loop {
        if counter >= config.iteration {
            break;
        }
        println!("Run program testing network");

        let output = Command::new("speedtest")
            .output()
            .expect("Failed to execute comman <speedtest> process");

        let result = String::from_utf8_lossy(&output.stdout);
        let info_speed_test = get_data_speedtest(result.to_string());

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config.filepath)
            .expect("Failed to open file");

        for data in &info_speed_test {
            if let Err(why) = writeln!(file, "{}", data) {
                panic!("Could not write file: {}", why);
            }
        }
        let now = Local::now();

        //Extrae la hora y minutos en formato de cadena
        let hours_minutes = now.format("%H:%M").to_string();
        if let Err(why) = writeln!(file, "{}", hours_minutes) {
            panic!("Could not write file: {}", why);
        }
        counter += 1;
        println!("Finish program network cycle {}", counter);
    }
}

fn calculate_file_speed_test(path: &str) {
    let contents = fs::read_to_string(path).unwrap();

    let re = Regex::new(r"[-]?\d*\.\d+").unwrap();

    let mut latency: Vec<f32> = vec![];
    let mut download: Vec<f32> = vec![];
    let mut upload: Vec<f32> = vec![];

    for line in contents.lines() {
        if line.contains("Latency") {
            for cap in re.captures_iter(line) {
                latency.push(cap[0].parse().unwrap());
            }
        } else if line.contains("Download") {
            for cap in re.captures_iter(line) {
                download.push(cap[0].parse().unwrap());
            }
        } else if line.contains("Upload") {
            for cap in re.captures_iter(line) {
                upload.push(cap[0].parse().unwrap());
            }
        }
    }

    println!("Latency average: {}", calculate_average(&latency));
    println!("Download average: {}", calculate_average(&download));
    println!("Upload average: {}", calculate_average(&upload));
}

fn calculate_average(values: &Vec<f32>) -> f32 {
    let sum: f32 = values.iter().sum();
    let len: f32 = values.len() as f32;

    sum / len
}

fn clear_file_txt(path: &str) -> std::io::Result<()> {
    OpenOptions::new().write(true).truncate(true).open(path)?;
    Ok(())
}
