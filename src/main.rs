extern crate horaire;
extern crate sms_freemobile_api;
extern crate chrono;

use horaire::source::{transilien::transilien, sncf::sncf, ratp::ratp};
use chrono::prelude::*;
use std::{env, str::FromStr};
use sms_freemobile_api::sms_service::SmsService;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 4 {
        let result = match args[1].as_str()
        {
            "transilien" => transilien(args[2].as_str()),
            "ratp" => ratp(args[2].as_str(), args[3].as_str()),
            "sncf" | "sncf_d" => sncf(args[2].as_str(), true),
            "sncf_a" => sncf(args[2].as_str(), false),
            _ => panic!("Unsupported argument")
        };
        match result {
            Ok(vec_time_lines) => {
                let wanted_direction = args[args.len() - 4].as_str();
                let time_str = args[args.len() - 3].as_str();
                let vec_time_h_m : Vec<&str> = time_str.split(':').collect();
                let h = u32::from_str(vec_time_h_m[0]).unwrap();
                let m = u32::from_str(vec_time_h_m[1]).unwrap();
                let margin = i64::from_str(args[args.len() - 2].as_str()).unwrap();
                let sms_user = args[args.len() - 1].as_str();
                let sms = SmsService::new("Accounts.toml");
                let opt_time_line = horaire::timelines::first_time_line_for_destination(vec_time_lines.iter(), wanted_direction);
                match opt_time_line {
                    Some(time_line) => {
                        let reference = Local::today().and_hms(h, m, 0);
                        let delay = time_line.get_seconds_difference_from_reference(&reference) / 60;
                        if delay > margin {
                            let _ = sms.sms_user(sms_user, format!("/!\\ Destination {} delay : {} minutes Expected_time {} Effective_time {}",
                                                                   wanted_direction, delay, time_str, time_line.get_time_string().as_str()).as_str());
                        } else {
                            let _ = sms.sms_user(sms_user, format!("OK: Destination {} delay : {} minutes Expected_time {} Effective_time {}",
                                                                   wanted_direction, delay, time_str, time_line.get_time_string().as_str()).as_str());
                        }
                    }
                    None => {
                        let _ = sms.sms_user(sms_user, format!("No destination : {} at {}", wanted_direction, time_str).as_str());
                    }
                }
            },
            _ => {panic!("Unable to retrieve data from website");}
        }
    }
    else {
        println!(concat!(
            "usage transilien [station] [wanted_direction] [time] [delay_margin] [sms_user]\n",
            "usage ratp [rer] [station] [wanted_direction] [time] [delay_margin] [sms_user]\n",
            "usage sncf       [station] [wanted_direction] [time] [delay_margin] [sms_user]\n",
            "usage sncf_d     [station] [wanted_direction] [time] [delay_margin] [sms_user]\n",
            "usage sncf_a     [station] [wanted_direction] [time] [delay_margin] [sms_user]\n"));
    }
}
