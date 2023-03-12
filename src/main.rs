use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;

fn main() {

    let mut counter = 0;

    loop {
        if counter >= 3 {
            break;
        }

        let output = Command::new("speedtest")
        .output()
        .expect("failed to execute process");

        let result = String::from_utf8_lossy(&output.stdout); 
       // println!("{}", result);
        let infoSpeedtest = get_data_speedtest(result.to_string());

        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ping_output.txt")
        .expect("failed to open file");

        for data in &infoSpeedtest {
            if let Err(why) = writeln!(file, "{}", data) {
                panic!("No se pudo escribir en el archivo: {}", why);
            }
        }

        sleep(Duration::from_secs(40));
          counter += 1;
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

    //println!("{:?}", results);
    results
}



