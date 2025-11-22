use serde::Deserialize;
use dotenvy::dotenv;
use chrono::Local;
use std::{env, thread, process::Command, time::Duration};

#[derive(Debug, Deserialize)]
struct Vnstat {
    interfaces: Vec<VnInterface>,
}

#[derive(Debug, Deserialize)]
struct VnInterface {
    name: String,
    traffic: Traffic,
    #[serde(flatten)]
    _other: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Traffic {
    total: Totals,
    day: Vec<Day>,
    month: Vec<Month>,
    #[serde(flatten)]
    _other: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Totals {
    rx: u64,
    tx: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Day {
    date: Date,
    rx: u64,
    tx: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Month {
    date: MonthDate,
    rx: u64,
    tx: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MonthDate {
    year: u32,
    month: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading .env...");
    dotenv().ok();

    let telegram_token: String = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
    let chat_id: String = env::var("CHAT_ID").expect("CHAT_ID not set");
    let interface_name = env::var("INTERFACE").expect("INTERFACE not set");

    let interval_hours: u64 = env::var("INTERVAL_HOURS")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("INTERVAL_HOURS must be a number");

    println!("Monitoring interface '{}'. Interval: {} hours", interface_name, interval_hours);

    loop {
        if let Err(e) = check_and_send(&telegram_token, &chat_id, &interface_name) {
            eprintln!("Error: {:?}", e);
        }

        thread::sleep(Duration::from_secs(interval_hours * 3600));
    }
}

fn check_and_send(token: &str, chat_id: &str, interface_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let output = Command::new("vnstat")
        .arg("--json")
        .output()?
        .stdout;

    let data: Vnstat = serde_json::from_slice(&output)?;

    let network_opt = data.interfaces.iter().find(|i| i.name == interface_name);
    let network = match network_opt {
        Some(n) => n,
        None => {
            eprintln!("Interface '{}' not found", interface_name);
            return Ok(());
        }
    };

    // Total akumulatif
    let total_rx = network.traffic.total.rx / 1024 / 1024;
    let total_tx = network.traffic.total.tx / 1024 / 1024;

    // Hari ini
    let today = network.traffic.day.last().unwrap();
    let today_rx = today.rx / 1024 / 1024;
    let today_tx = today.tx / 1024 / 1024;

    // Bulan ini
    let this_month = network.traffic.month.last().unwrap();
    let month_rx = this_month.rx / 1024 / 1024;
    let month_tx = this_month.tx / 1024 / 1024;

    let message = format!(
        "Network usage for interface '{}':\n\
        Today: Download {} MB | Upload {} MB\n\
        Month: Download {} MB | Upload {} MB\n\
        Total: Download {} MB | Upload {} MB\n\
        Timestamp: {}",
        interface_name,
        today_rx, today_tx,
        month_rx, month_tx,
        total_rx, total_tx,
        timestamp
    );

    send_to_telegram(token, chat_id, &message)?;

    println!("[{}] Interface '{}': Today: {}MB/{}MB | Month: {}MB/{}MB | Total: {}MB/{}MB",
        timestamp, interface_name,
        today_rx, today_tx,
        month_rx, month_tx,
        total_rx, total_tx
    );

    Ok(())
}

fn send_to_telegram(telegram_token: &str, chat_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", telegram_token);
    let _res = reqwest::blocking::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message
        }))
        .send()?;
    Ok(())
}
