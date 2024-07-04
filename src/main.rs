
#![allow(unused_parens)]

use std::env::args;
use std::time::Instant;
use std::io::{stdout, Write};

use clap::Parser;
//use itertools::*;

#[derive(Parser, Debug)]
#[command(disable_help_flag(true))]
struct Args {

    /// Switches
    #[arg(short, required(false), default_value_t=("0".to_string()))]
    minutes: String,
    #[arg(short = 'H', required(false), default_value_t=("0".to_string()))]
    hours: String,
    #[arg(short, required(false), default_value_t=("0".to_string()))]
    days: String,
    #[arg(short = 'h', required(false), default_value_t=false)]
    show_help: bool,
    #[arg(short = 't', required(false), default_value_t=(false))]
    timestamp: bool,
    #[arg(required(false), default_value_t=("0".to_string()))]
    time: String,
}

use crossterm::{
    cursor::MoveToColumn,
    terminal::{Clear, ClearType::UntilNewLine},
    QueueableCommand
};

macro_rules! set_panic {
    ($msg:expr) => {
        std::panic::set_hook(Box::new(|_| {
            println!("{}", $msg);            
        }));
    }
}

macro_rules! set_error_panic {
    ($msg:expr) => {
        set_panic!(String::from(HELP_MSG) + "\n" + $msg);
    }
}

fn parse_timestamp(timestamp: String) -> i128 {


    let mut ms = 0;
    let mut sec = 0;
    let mut min = 0;
    let mut hours = 0;
    let mut days = 0;

    let mut values = vec![&mut sec, &mut min, &mut hours, &mut days];
    
    let mut split = timestamp
        .split(":")
        .collect::<Vec<&str>>();

    split.reverse();
    let mut decimal_split = split[0]
        .split(".")
        .collect::<Vec<&str>>();
    decimal_split.reverse();

    // Parse after decimal point //
    if decimal_split.len() > 2 {
        set_error_panic!("Error: only one decimal point allowed.");
        panic!();
    } else if decimal_split.len() == 2 {
        let mut milliseconds = decimal_split[0].to_owned();
        
        while milliseconds.chars().count() < 3 {
            milliseconds += "0";
        }

        { log::debug!("{:?}", &milliseconds); }

        ms += match milliseconds.parse::<u128>() {
            Ok(num) => num,
            Err(_) => { set_error_panic!("Error: Invalid timestamp -- decimal portion.");
                        panic!(); },
        };

        { log::debug!("{:?}", &ms); }
    }

    
    // Parse everything else //
    let mut pointer_idx = 0;
    if split.len() > 1 {
        for section in split.iter() {

            if pointer_idx >= values.len() { break; };

            *(values[pointer_idx]) = match section.split(".")
                .collect::<Vec<&str>>()[0]
                .trim_start_matches(&['[']) // Just in case someone gets the idea to copy the 
                .trim_end_matches(&[']']) // display format
                .parse::<u128>() {
                Ok(num) => num,
                Err(_) => { set_error_panic!("Error: Invalid timestamp -- use numbers and ':' character only.");
                            panic!(); },
            };

            { log::debug!("{:?}", &*values[pointer_idx]); }

            pointer_idx += 1;
        }
    }


    ( ms + 
    (sec * 1000) + 
    (min * 1000 * 60) + 
    (hours * 1000 * 60 * 60) + 
    (days * 1000 * 60 * 60 * 24) ) as i128
}

fn format_time(millis: i128) -> String {

    let mut remaining = millis;
    let mut days = 0;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    while remaining >= 1000 {
        let mut divisor = 1000;
        let mut val_ptr = &mut seconds;
        if remaining >= 1000 * 60 * 60 * 24 {
            divisor = 1000 * 60 * 60 * 24;
            val_ptr = &mut days
        }
        else if remaining >= 1000 * 60 * 60 {
            divisor = 1000 * 60 * 60;
            val_ptr = &mut hours;
        }
        else if remaining >= 1000 * 60 {
            divisor = 1000 * 60;
            val_ptr = &mut minutes;
        }
        
        let remainder = remaining % divisor;
        let whole_quotient = (remaining - remainder) / divisor;
        remaining -= whole_quotient * divisor;
        *val_ptr += whole_quotient;
    }
 
    format!("[{:0>2}:]{:0>2}:{:0>2}:{:0>2}.{:0>3}", days, hours, minutes, seconds, remaining)
}

const HELP_MSG: &str = "Usage: `sleepview [SWITCH] DURATION`
 DURATION: the amount of time to count down in seconds. Can be specified in combination with switches, or omitted entirely with switches present. Using a timestamp disables other switches.
 SWITCHES:
-h :\tShow this help message and exit.
-d :\tSpecify days.
-H :\tSpecify hours.
-m :\tSpecify minutes.
-t :\tSpecify a timestamp, in the form (D)D:(H)H:(M)M:(S)S(.DEC) -- days, hours, minutes, seconds, decimal portion.";

fn main() -> () {
   
    // Should be the first thing done for maximum accuracy
    let start = Instant::now();
    
    #[cfg(debug_assertions)]
    env_logger::init();
    set_panic!(HELP_MSG);
 
    // Primarily initializes `time_spent`
    let mut time_spent = start.elapsed().as_millis();

    if args().count() < 2 {
        set_error_panic!("Error: needs at least one argument to specify duration.");
        panic!();
    }
    /*
    else if args().count() > 3 {
        set_error_panic!("Error: too many arguments.");
        panic!()
    }
    */
    { log::debug!("{:?}", Args::parse()); }

    let clapargs = match Args::try_parse() {
        Ok(args) => {
            args
        },
        Err(e) if format!("{:?}", e.kind()) == "InvalidValue" => {
            set_error_panic!("Error: invalid or missing value for argument.");        
            panic!();
        },
        Err(_) => {
            set_error_panic!("Error: invalid argument(s).");
            panic!();
        },
    };

    if clapargs.show_help { 
        panic!()
    };

    let possible_switches = vec![clapargs.minutes, clapargs.hours, clapargs.days];

    let factors = vec![
        1,
        60, 
        60 * 60, 
        60 * 60 * 24
    ];
 

    let mut target = 0;

    if !clapargs.timestamp {
        for possible_input_idx in 0..=possible_switches.len() {

            if possible_input_idx == 0 {
                target += match clapargs.time.parse::<f64>() {
                    Ok(num) => (num * 1000.0) as i128,
                    Err(_) => target,
                };
            } else {
                target += match possible_switches[possible_input_idx-1].parse::<f64>() {
                    Ok(num) => { { log::debug!("idx = {:?}", &possible_input_idx); }
                                 { log::debug!("num = {num}"); }
                                 (num * 1000.0) as i128 * factors[possible_input_idx] },
                    Err(_) => target,
                };

            } 
        }

    } else {
        // should be last item in possible_switches
        { log::debug!("{:?} {:?}", "Parsing timestamp.", clapargs.time); }
        target = parse_timestamp(clapargs.time);
    }

    let mut time_over = false;
    while time_spent as i128 <= target as i128 + 100i128 {
        
        let _ = stdout().queue(MoveToColumn(0));

        let difference = target as i128-time_spent as i128;
        if difference < 0i128 {
            print!("{}", format_time(0i128));
            time_over = true;

        } else {
            print!("{}", format_time(difference));
        }
        
        let _ = stdout().queue(Clear(UntilNewLine));

        let _ = stdout().flush();

        if time_over { break; }

        std::thread::sleep(std::time::Duration::from_micros(800));
        time_spent = start.elapsed().as_millis();

    }

    println!();
    ()
}
