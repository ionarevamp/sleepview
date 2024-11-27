#![allow(clippy::print_with_newline)]
#![allow(unused_parens)]

mod json;
mod help;

use help::*;

use std::time::Instant;
use std::time::SystemTime;
use std::io::{stdout, Write};
#[cfg(debug_assertions)]
use std::fmt;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, MoveRight},
    style::{SetForegroundColor, Print, Color::{*}},
    QueueableCommand
};

use json::produce_json;

use clap::Parser;
//use itertools::*;

#[derive(Parser, Debug)]
#[command(disable_help_flag(true))]
struct Args {
    /// Switches
    #[arg(short, required(false), default_value_t=(false))]
    no_newline: bool,
    #[arg(short, required(false), default_value_t=false)]
    full: bool,
    #[arg(short, required(false), default_value_t=false)]
    json: bool,
    #[arg(short, required(false), default_value_t=("0".to_string()))]
    minutes: String,
    #[arg(short = 'H', required(false), default_value_t=("0".to_string()))]
    hours: String,
    #[arg(short, required(false), default_value_t=("0".to_string()))]
    days: String,
    #[arg(short = 'h', long="help", required(false), default_value_t=false)]
    show_help: bool,
    #[arg(short = 't', required(false), default_value_t=(String::new()))]
    timestamp: String,
    #[arg(required(false), default_values_t=(vec!["0".to_string()]))]
    time: Vec<String>,
    #[arg(short, required(false), default_value_t=200.0)]
    rate: f64,
    #[arg(short = 'R', required(false), default_value_t='0')]
    resolution: char,
    #[arg(short, required(false), default_value_t=false)]
    up: bool,
    #[arg(short, long="output", required(false), default_value_t=String::new())]
    output_file: String,
    #[arg(short = 'I', long="instant", required(false), default_value_t=false)]
    instant: bool,
}

#[cfg(debug_assertions)]
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(target_os = "windows"))]
        let new_line = "\n";
        #[cfg(target_os = "windows")]
        let new_line = "\r\n";
        write!(f, "Args {{{5} minutes: {:?}{5} hours: {:?}{5} days: {:?}{5} show_help: {:?}{5} timestamp: {:?}{5}",
            self.minutes,
            self.hours,
            self.days,
            self.show_help,
            self.timestamp,
            new_line
        )?;

        write!(f, " time: {}{}}}", self.time[0], new_line)
    }
}

macro_rules! set_panic {
    ($msg:expr) => {
        std::panic::set_hook(Box::new(|_| {
            println!("{}", $msg);            
        }));
    }
}

macro_rules! set_error_panic {
    ($msg:expr) => {
        set_panic!(String::from("sleepview v") + env!("CARGO_PKG_VERSION") + "\n" + HELP_MSG + "\n" + $msg);
    }
}

#[inline(always)]
fn new_line() {
    #[cfg(target_os = "windows")]
    print!("\r");
    print!("\n");

}

#[inline(never)]
fn sleep_minimal(sleep_amount: u64, target: i128, start: Instant) {

    let mut sleep_amount = sleep_amount;
    loop {
        if (target * 1000i128 - start.elapsed().as_micros() as i128) < 500i128 {
            std::thread::sleep(std::time::Duration::from_micros(8000));
            //std::hint::spin_loop();
            break;
        }
        if (sleep_amount as i128) < target * 1000i128 - start.elapsed().as_micros() as i128 {
            std::thread::sleep(std::time::Duration::from_nanos(sleep_amount * 1000u64));
            break;
        } else {
            sleep_amount = (sleep_amount * 12u64) / 13u64;
        }
    }
}

#[inline(always)]
fn check_system_time(sleep_amount: u64, target: i128, start: SystemTime) {

    let mut sleep_amount = sleep_amount;
    loop {
        if (target * 1000i128 - start.elapsed().unwrap().as_micros() as i128) < 500i128 {
            std::thread::sleep(std::time::Duration::from_micros(8000));
            //std::hint::spin_loop();
            break;
        }
        if (sleep_amount as i128) < target * 1000i128 - start.elapsed().unwrap().as_micros() as i128 {
            std::thread::sleep(std::time::Duration::from_nanos(sleep_amount * 1000u64));
            break;
        } else {
            sleep_amount = (sleep_amount * 12u64) / 13u64;
        }    
    }
   
}

#[inline(always)]
fn parse_timestamp(timestamp: String) -> i128 {

    let mut ms = 0;
    let mut sec = 0;
    let mut min = 0;
    let mut hours = 0;
    let mut days = 0;

    let values = [&mut sec, &mut min, &mut hours, &mut days];
    
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
    }
    if decimal_split.len() == 2 {
        let mut milliseconds = decimal_split[0].to_owned();
        
        while milliseconds.chars().count() < 3 {
            milliseconds += "0";
        }

        { log::debug!("timestamp milliseconds: {:?}", &milliseconds); }

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
                .trim_start_matches(['[']) // Just in case someone gets the idea to copy the 
                .trim_end_matches([']']) // display format
                .parse::<u128>() {
                    Ok(num) => num,
                    Err(_) => { set_error_panic!("Error: Invalid timestamp -- use numbers and ':' character only.");
                            panic!(); },
            };

            { log::debug!("timestamp: set number to {:?}", &*values[pointer_idx]); }

            pointer_idx += 1;
        }
    }


    ( ms + 
    (sec * 1000) + 
    (min * 1000 * 60) + 
    (hours * 1000 * 60 * 60) + 
    (days * 1000 * 60 * 60 * 24) ) as i128
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn format_time(millis: i128, previous_millis: i128, format_width: usize, resolution: usize, as_json: bool, output: &mut impl Write, is_file: bool, first_draw: bool) {

    let mut remaining = millis;
    let mut days = 0;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    let mut prev_remaining = previous_millis;
    let mut prev_days = 0;
    let mut prev_hours = 0;
    let mut prev_minutes = 0;
    let mut prev_seconds = 0;

    
    #[cfg(debug_assertions)]
    log::debug!("format_time: millis = {millis}, previous_millis = {previous_millis}");

    while remaining >= 1000 {
        let mut divisor = 1000;
        let mut val_ptr = &mut seconds;

        if remaining >= 1000 * 60 * 60 * 24 {
            divisor = 1000 * 60 * 60 * 24;
            val_ptr = &mut days;
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
        let multiple = remaining - remainder;
        remaining -= multiple;

        *val_ptr += multiple / divisor;
    }
    
    while prev_remaining >= 1000 {
        let mut divisor = 1000;
        let mut val_ptr = &mut prev_seconds;

        if prev_remaining >= 1000 * 60 * 60 * 24 {
            divisor = 1000 * 60 * 60 * 24;
            val_ptr = &mut prev_days;
        }
        else if prev_remaining >= 1000 * 60 * 60 {
            divisor = 1000 * 60 * 60;
            val_ptr = &mut prev_hours;
        }
        else if prev_remaining >= 1000 * 60 {
            divisor = 1000 * 60;
            val_ptr = &mut prev_minutes;
        }
        
        let remainder = prev_remaining % divisor;
        let multiple = prev_remaining - remainder;
        prev_remaining -= multiple;

        *val_ptr += multiple / divisor;

        #[cfg(debug_assertions)]
        let _ = (*output).queue(Print(format!("adding amount {} at divisor {}\n", multiple / divisor, divisor)));

    }
    set_error_panic!("Failed inside `format_time()`");
 
    let values = [
        remaining,
        seconds,
        minutes,
        hours,
        days
    ];

    let prev_values = [
        prev_remaining,
        prev_seconds,
        prev_minutes,
        prev_hours,
        prev_days
    ];

    // Output json OR continue with normal formatting
    if as_json {
        produce_json(values, format_width, resolution, output);
        return;
    }
    if !is_file {

        // Essentially, if a field is different or it's the first time printing, print the data,
        // but otherwise just move the cursor forward

        let _ = (*output).queue(SetForegroundColor(Reset));
        for i in (resolution..format_width).rev() {
           
            #[cfg(debug_assertions)]
            {
                let _ = (*output).queue(Print(format!("{}: prev: {} now: {} \n", 
                    match i {
                        0 => "millis",
                        1 => "seconds",
                        2 => "minutes",
                        3 => "hours",
                        4 => "days",
                        _ => "unknown",
                    },
                prev_values[i], values[i])));
            }

            #[cfg(not(debug_assertions))]
            if values[i] != prev_values[i] || first_draw {
                match i {
                    0 => { // millis
                        let _ = (*output).queue(SetForegroundColor(Grey));
                        let _ = (*output).queue(Print(format!(".{:0>3}", values[i])));
                    },
                    1 => { // seconds
                        let _ = (*output).queue(Print(format!("{:0>2}", values[i])));
                    },
                    4 => { // days
                        let _ = (*output).queue(Print(format!("[{:0>2}", values[i])));
                    },
                    _ => { // minutes, hours
                        let _ = (*output).queue(Print(format!("{:0>2}", values[i])));
                    },
                }
                if i > resolution && i > 1 {
                    let _ = (*output).queue(Print(":"));
                }
                if i == 4 {
                    let _ = (*output).queue(Print("]"));
                }
            }

            if values[i] == prev_values[i] && !first_draw {
                #[cfg(debug_assertions)]
                {
                    let _ = (*output).queue(Print("(caught same value, would skip printing)\n"));
                }

                let mut offset = 0;
                if i > 0 {
                    offset += 2;
                }
                if i > resolution && i > 1 {
                    offset += 1;
                }             
                if i == 4 {
                    offset += 2;
                }
                if offset > 0 {
                    #[cfg(not(debug_assertions))]
                    let _ = (*output).queue(MoveRight(offset));
                }
            }
        }
    } else {

        //VERSION 1 -- Avg. total write time: 0.934897854006473ms

        let mut fields: Vec<String> = Vec::with_capacity(format_width-resolution);  
        for (i, value) in (values[resolution..format_width]).iter().enumerate().rev() {
            match i+resolution {
                0 => { // millis
                    fields.push(format!(".{:0>3}", value));
                },
                1 => { // seconds
                    fields.push(format!("{:0>2}", value));
                },
                4 => { // days
                    fields.push(format!("[{:0>2}:]", value));
                },
                _ => { // minutes, hours
                    fields.push(format!("{:0>2}:", value));
                },
            }
        }

        let _ = output.write_fmt(format_args!("{}", fields.concat()));
        

        // VERSION 2 -- Avg. total write time: ~0.9970945037976796ms
        /* 
        let max = values.len()-1;
        match format_width {
            0..=2 => {
                let _ = write!(output,
                        "{:0>2}.{:0>3}",
                        values[max-1],
                        values[max]
                    );
            },
            3 => {
                let _ = write!(output,
                        "{:0>2}:{:0>2}.{:0>3}",
                        values[max-2],
                        values[max-1],
                        values[max]
                    );
            },
            4 => {
                let _ = write!(output,
                        "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
                        values[max-3],
                        values[max-2],
                        values[max-1],
                        values[max]
                    );
            }
            _ => {
                let _ = write!(output,
                        "[{:0>2}:]{:0>2}:{:0>2}:{:0>2}.{:0>3}",
                        values[max-4],
                        values[max-3],
                        values[max-2],
                        values[max-1],
                        values[max]
                    );
            },
        }
        */
        
    }


//    format!("[{:0>2}:]{:0>2}:{:0>2}:{:0>2}.{:0>3}", days, hours, minutes, seconds, remaining)
}


fn main() {
   
    // Should be the first thing done for maximum accuracy
    let instant_start = Instant::now();
    let system_time_start = SystemTime::now();
   
    #[cfg(debug_assertions)]
    env_logger::init();
    // Default help message
    set_error_panic!("");
 
    { log::debug!("{:#?}", Args::parse()); }

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

    let sleep_amount = (1_000_000.0 / clapargs.rate) as u64;

    let possible_switches = [clapargs.minutes, clapargs.hours, clapargs.days];

    let factors: [i128; 4] = [
        1,
        60, 
        60 * 60, 
        60 * 60 * 24
    ];
 

    let mut target = 0;

    if clapargs.timestamp.chars().count() > 0 {
        { log::debug!("Parsing timestamp. {:?}", clapargs.time[0]); }
        target = parse_timestamp(clapargs.timestamp.clone());
    } else {

        { log::debug!("Checking for duration switches..."); }
        // Clap argument parsing
        for possible_input_idx in 0..possible_switches.len() {


            target += match possible_switches[possible_input_idx].parse::<f64>() {
                Ok(num) => { { log::debug!("possible_input_idx = {:?}", possible_input_idx); }
                             { log::debug!("provided value = {num}"); }
                             (num * 1000.0) as i128 * factors[possible_input_idx+1] },
                Err(_) => { 0 },
            };

        }
    }

    // Fallback parsing -- GNU sleep imitation
    //if target == 0i128 {
        let mut factor_idx;

        {
            log::debug!("Using trailing arguments as fallback arguments... {:?}", clapargs.time);
            log::debug!("... Adding to existing target time.");
        }

//        for arg in (&osargs[1..]).iter() {
        for arg in clapargs.time.iter() {       
            let len = arg.chars().count();
            target += ( match arg.parse::<f64>() {
                Ok(num) => { factor_idx = 0; num },
                _ => {

                    { log::debug!("truncated argument {:?}", &arg[..len-1]); }

                    match (arg[..len-1]).to_string().parse::<f64>() {
                        Ok(num) => {

                            { log::debug!("num ok: {:?}",num); }

                            if let Some(suffix) = arg.chars().nth(len-1) { // 

                                { log::debug!("suffix is {:?}",suffix); }

                                match suffix {
                                    's' | 'S' => { factor_idx = 0; num },
                                    'm' | 'M' => { factor_idx = 1; num },
                                    'h' | 'H' => { factor_idx = 2; num },
                                    'd' | 'D' => { factor_idx = 3; num },
                                    _ => { set_error_panic!("Error: invalid suffix.");

                                          { log::debug!("unrecognized suffix {:?}", &suffix); }

                                            panic!(); },
                                }
                            } else {
                                set_error_panic!("Error: argument somehow has a length of zero?");
                                panic!();
                            }
                        },
                        Err(_) => { set_error_panic!("Error: invalid number.");
                                    panic!(); },
                    }
                },
            } * 1000.0 ) as i128 * factors[factor_idx];
        }
    //}

    { log::debug!("total target time is {:?} milliseconds", target); }

    if target == 0 {
        { log::debug!("Exiting due to target time of zero."); }
        set_panic!("");
        panic!();
    }

    let mut instant_time_spent = instant_start.elapsed().as_millis();
    let mut system_time_spent = system_time_start.elapsed().unwrap().as_millis();

    let mut last = match clapargs.up {
        false => {
            if clapargs.instant {
                target-instant_time_spent as i128
            } else {
                target-system_time_spent as i128
            }
        },
        true => target,
    };


    let format_width =
        if clapargs.full {
            5
        } else {
            let mut width = 2;
            for (i,factor) in factors.iter().enumerate() {
                if target > factor * 1000 {
                    width = i+2;
                }
            }
            width
        };

    { log::debug!("format_width = {}", format_width); }
    set_error_panic!("Unknown error.");
    
    let resolution = match clapargs.resolution {
        'm' | '0' => 0,
        's' | 'S' | '1' => 1,
        'M' | '2' => 2,
        'h' | 'H' | '3' => 3,
        'd' | 'D' | '4' => 4,
        _ => {
            set_error_panic!("Error: invalid resolution setting.");
            panic!();
        }
    };

    #[allow(clippy::comparison_chain)]
    if resolution == format_width {
        let _ = stdout().queue(Print("Warning: resolution setting prevents useful output"));
        new_line();
    } else if resolution > format_width {
        set_error_panic!("Error: resolution too coarse for the amount of time provided. (Try using a different resolution or using the '-f' switch.)");
        panic!();
    }

    { log::debug!("Resolution set to {} ({})", resolution,
        match resolution {
            0 => "milliseconds",
            1 => "seconds",
            2 => "minutes",
            3 => "hours",
            4 => "days",
            _ => "unknown",
        }); }

    let is_file = !matches!(clapargs.output_file.as_str(), "" | "-");
    { log::debug!("Output determined. is_file = {:?}", is_file); }

    let output_buf = &mut std::io::stdout().lock();

    let output_tmp = clapargs.output_file.clone() + ".tmp";
   
    #[cfg(debug_assertions)]
    let mut creation_times = Vec::new(); 
    #[cfg(debug_assertions)]
    let mut rename_times = Vec::new();
    #[cfg(debug_assertions)]
    let mut total_write_times = Vec::new();

    if is_file {
        println!("Running.");
    }


    // MAIN LOOP
    let mut time_over = false;
    let mut first_draw = true;
    let mut time_spent = if clapargs.instant {
        &mut instant_time_spent
    } else {
        &mut system_time_spent
    };
    loop {
        if !is_file {
            let _ = stdout().queue(MoveToColumn(0));
        }


        let mut difference = match clapargs.up {
            false => target-*time_spent as i128,
            true => *time_spent as i128,
        };
        
        if !clapargs.up {
            if difference <= 0i128 {
                difference = 0;
                time_over = true;
            }
        } else if difference >= target {
            difference = target;
            time_over = true;
        }


        if is_file {

            #[cfg(debug_assertions)]
            let pre_write = Instant::now();

            #[cfg(debug_assertions)]
            let pre_file = Instant::now();


            let path = std::path::Path::new(&output_tmp);
            let mut file = std::fs::File::create(path).unwrap_or_else(|_| {
                set_error_panic!("Error: Could not create tmp file");
                panic!();
            });
        
            #[cfg(debug_assertions)]
//            creation_times.push(pre_file.elapsed().as_micros());


            format_time(difference, last, format_width, resolution, clapargs.json, { 

                &mut file
            }, is_file, false);

            #[cfg(debug_assertions)]
            let pre_rename = Instant::now();

            std::fs::rename(
                std::path::Path::new(&output_tmp),
                std::path::Path::new(&clapargs.output_file)
            ).unwrap_or_else(|_| {
                set_error_panic!("Error: Could not rename tmp file");
                panic!();
            });

            #[cfg(debug_assertions)]
            {
  //              rename_times.push(pre_rename.elapsed().as_micros());
    //            total_write_times.push(pre_write.elapsed().as_micros());
            }

        } else {
            format_time(difference, last, format_width, resolution, clapargs.json, output_buf, is_file, first_draw);
            first_draw = false;
        }

        if !is_file {
        //    let _ = stdout().queue(Clear(UntilNewLine));
            new_line();
            let _ = output_buf.flush();
        }

       
        if time_over {
            if !is_file {
                let _ = stdout().queue(MoveUp(1));
            }
            break;
        }

        if clapargs.instant {
            sleep_minimal(sleep_amount, target, instant_start);
        } else {
            check_system_time(sleep_amount, target, system_time_start);
        }


        if !clapargs.json && !is_file {
            let _ = stdout().queue(MoveUp(1));
        }

        *time_spent = if clapargs.instant {
            instant_start.elapsed().as_millis()
        } else {
            system_time_start.elapsed().unwrap().as_millis()
        };

        // check for consistency...
        if last < difference {
            let _ = stdout().write_all(b"\x07");
        }

        #[cfg(debug_assertions)]
        let _ = stdout().write_fmt(format_args!("difference: {}, last: {}\n", difference, last));

        last = difference;
    }

    if !is_file {
        let _ = stdout().queue(SetForegroundColor(Reset));
        if !clapargs.no_newline {
            new_line();
        }
        let _ = output_buf.flush();
    } else {
        println!("Done.");
    }

    #[cfg(debug_assertions)]
    if is_file {
        let creation_count = creation_times.len();
        log::debug!("Average file creation time: {}ms",
            creation_times
            .iter()
            .sum::<u128>() as f64 / 1000.0 / creation_count as f64);

        let rename_count = rename_times.len();
        log::debug!("Average file rename time: {}ms",
            rename_times
            .iter()
            .sum::<u128>() as f64 / 1000.0 / rename_count as f64);

        let write_count = total_write_times.len();
        log::debug!("Average total file write time: {}ms",
            total_write_times
            .iter()
            .sum::<u128>() as f64 / 1000.0 / write_count as f64);

    }
}
