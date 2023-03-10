#[macro_use]
extern crate serde_json;


use yansi::Paint;
use chrono::Local;
use log::{self, LevelFilter};
use env_logger::Builder;
use std::env;
use std::io::Write;
use cmd::{encode, decode, generate};
use crate::error::log_backtrace;

pub mod cli;
pub mod aes;
pub mod json;
pub mod cmd;
pub mod error;


pub fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let mut style = formatter.style();
        style.set_bold(true);

        let tar = Paint::blue("miner keys").bold();

        match record.level() {
            log::Level::Error => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::red("Error").bold(),
                Paint::red(record.args()).wrap()
            ),
            log::Level::Warn => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::yellow("Warning").bold(),
                Paint::yellow(record.args()).wrap()
            ),
            _ => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::blue(record.level()).bold(),
                Paint::blue(record.args()).wrap()
            ),
        }
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}


#[tokio::main]
async fn main() {
    init_logger();
    let matches = cli::build_cli().get_matches();

    let res = match matches.subcommand() {
        ("encode", Some(matches)) => {
            let password = rpassword::read_password_from_tty(Some("> password: ")).unwrap_or("".to_string());
            let message = matches.value_of("message").unwrap_or_default();

            encode(message, password.as_str())
        }
        ("decode", Some(matches)) => {
            let password = rpassword::read_password_from_tty(Some("> password: ")).unwrap_or("".to_string());
            let message = matches.value_of("message").unwrap_or_default();
            decode(message, password.as_str())
        }
        ("generate", Some(matches)) => {
            //  words: &str, format: &str, number: &i32
            let words = matches.value_of("words").unwrap_or_default();
            let format = matches.value_of("format").unwrap_or_default();
            let network = matches.value_of("network").unwrap_or_default();
            let amount = matches.value_of("amount").unwrap_or_default().parse::<i32>().unwrap();
            generate(words,format,amount,network)
        }
        _ => unreachable!(),
    };
    if let Err(e) = res {
        log_backtrace(&e);
        std::process::exit(101);
    }
}
