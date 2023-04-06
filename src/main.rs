use crate::metric::Metric;
use crate::setting::Config;
use chrono::prelude::*;
use plotters::prelude::BitMapBackend; 
use plotters::prelude::ChartBuilder;
use plotters::prelude::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style::IntoFont;
use plotters::style::RED;
use plotters::style::WHITE;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::process::Command;

mod metric;
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
        code if code == &config.command_show => average_metrics(&config.filepath),
        code if code == &config.command_cls => match clear_file_txt(&config.filepath) {
            Ok(_) => println!("File cleared successfully"),
            Err(e) => eprint!("Error clearing file: {}", e),
        },
        code if code == &config.command_metric => match create_graphic_metric(&config.filepath) {
            Ok(_) => println!("Metric image generate successfully"),
            Err(e) => eprint!("Error to generate metric image: {}", e),
        },
        code if code == &config.command_help => help(config),

        _ => {
            println!("Error: Command '{}' not exist", command);
            println!("Run help command to see available commands");
        }
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
        let mut hours_minutes = now.format("%H:%M").to_string();
        hours_minutes = "Time: ".to_owned() + &hours_minutes;
        if let Err(why) = writeln!(file, "{}", hours_minutes) {
            panic!("Could not write file: {}", why);
        }
        counter += 1;
        println!("Finish program network cycle {}", counter);
    }
}

fn get_infomation_file(path: &str) -> Metric {
    let contents = fs::read_to_string(path).unwrap();

    let re = Regex::new(r"[-]?\d*\.\d+").unwrap();
    let re_time = Regex::new(r"Time: (\d{2}:\d{2})").unwrap(); // agregado

    let mut metric: Metric = Metric::new();

    for line in contents.lines() {
        if line.contains("Latency") {
            for cap in re.captures_iter(line) {
                metric.latency.push(cap[0].parse().unwrap());
            }
        } else if line.contains("Download") {
            for cap in re.captures_iter(line) {
                metric.download.push(cap[0].parse().unwrap());
            }
        } else if line.contains("Upload") {
            for cap in re.captures_iter(line) {
                metric.upload.push(cap[0].parse().unwrap());
            }
        } else if let Some(caps) = re_time.captures(line) {
            let time_str = caps[1].to_owned();
            let parts: Vec<&str> = time_str.split(":").collect();

            if let [hours, minutes] = parts.as_slice() {
                // Convert both hours and minutes to floats or decimals
                let hours_f = hours.parse::<f32>().unwrap_or(0.0);
                let minutes_f = minutes.parse::<f32>().unwrap_or(0.0);

                metric.time.push(hours_f + (minutes_f / 60.0));
            }
        }
    }

    return metric;
}

fn average_metrics(path: &str) {
    let metric: Metric = get_infomation_file(path);
    println!("Latency average: {}", calculate_average(&metric.latency));
    println!("Download average: {}", calculate_average(&metric.download));
    println!("Upload average: {}", calculate_average(&metric.upload));
    println!("Time start {}", metric.time[0]);
    println!("Time end {}", metric.time[metric.time.len() - 1]);
}

fn create_graphic_metric(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metric: Metric = get_infomation_file(path);

    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Connection vs tiempo", ("Arial", 30).into_font())
        .build_cartesian_2d(0.0..100.0, 0.0..100.0)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            metric
                .time
                .iter()
                .zip(metric.download.iter())
                .map(|(x, y)| (x.round() as f64, y.round() as f64)),
            &RED,
        ))
        .unwrap();

    Ok(())
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

fn help(config: Config) {
    println!("-----------------------------------------------");
    println!("Commands action program");
    println!("-----------------------------------------------");
    println!("  Execute create report:<{}>", config.command_run);
    println!("  Execute show report:<{}>", config.command_show);
    println!(
        "  Execute generate metric image:<{}>",
        config.command_metric
    );
    println!("  Execute clear report:<{}>", config.command_cls);
    println!("-----------------------------------------------");
}
