use std::process::Command;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use std::env;
use chrono::prelude::*;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command : String = args[1].parse().unwrap();
        if command == "data" {calculate_file_speed_test();}    
        else {println!("Error command:{} not exist",command);}

    }else {
        println!("Ejecuta speedTest");
        run_speed_test();
    }
   
}

fn get_data_speedtest(input:String)-> Vec<String>{
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

fn run_speed_test(){
    let mut counter = 0;

    loop {
        if counter >= 3 {
            break;
        }

        let output = Command::new("speedtest")
        .output()
        .expect("failed to execute process");

        let result = String::from_utf8_lossy(&output.stdout); 
        let info_speed_test = get_data_speedtest(result.to_string());

        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ping_output.txt")
        .expect("failed to open file");

        for data in &info_speed_test {
            if let Err(why) = writeln!(file, "{}", data) {
                panic!("No se pudo escribir en el archivo: {}", why);
            }
        }
        let now = Local::now();

        //Extrae la hora y minutos en formato de cadena
        let hora_minutos = now.format("%H:%M").to_string();
        if let Err(why) = writeln!(file, "{}", hora_minutos) {
            panic!("No se pudo escribir en el archivo: {}", why);
        }

        sleep(Duration::from_secs(40));

          counter += 1;
     }
}

fn calculate_file_speed_test(){
    let contents = fs::read_to_string("ping_output.txt").unwrap(); 

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


