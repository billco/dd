use chrono::prelude::*;
use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str;

extern crate chrono;
struct Daymm {
    max: usize,
    min: usize,
}

#[derive(Copy, Clone)]
struct Hour {
    time: NaiveDateTime,
    temperature: f32,
    apparent_temperature: f32,
    dew_point: f32,
    humidity: f32,
    pressure: f32,
    precip_intensity: f32,
    precip_probability: f32,
    precip_type: usize,
    wind_speed: f32,
    wind_gust: f32,
    wind_bearing: f32,
    cloud_cover: f32,
    uv_ix: f32,
    visibility: f32,
    ozone: f32,
    summary: usize,
    icon: usize,
}

// impl Hour {

fn hrindex(h: Hour, index: usize) -> f32 {
    match index {
        // 0 => &h.time,
        1 => h.temperature,
        2 => h.apparent_temperature,
        3 => h.dew_point,
        4 => h.humidity,
        5 => h.pressure,
        6 => h.precip_intensity,
        7 => h.precip_probability,
        8 => h.wind_speed,
        9 => h.wind_gust,
        10 => h.wind_bearing,
        11 => h.cloud_cover,
        12 => h.uv_ix,
        13 => h.visibility,
        // 15 => &h.summary,
        // 8 => &h.precip_type,
        // 16 => &h.icon,
        _ => panic!("Invalid Vector3d index:"),
    }
}
fn main() {
    use std::time::Instant;
    let start = Instant::now();
    let mut day_break: Vec<usize> = vec![0;4];

    let mut daymm: Vec<Daymm> = Vec::new();

    fn new() -> Hour {
        Hour {
            time: NaiveDateTime::from_timestamp(0i64, 1u32),
            temperature: 0.,
            apparent_temperature: 0.,
            dew_point: 0.,
            humidity: 0.,
            pressure: 0.,
            precip_intensity: 0.,
            precip_probability: 0.,
            wind_speed: 0.,
            wind_gust: 0.,
            wind_bearing: 0.,
            cloud_cover: 0.,
            uv_ix: 0.,
            visibility: 0.,
            ozone: 0.,
            precip_type: 0,
            summary: 0,
            icon: 0,
        }
    }
    let mut hr: Vec<Hour> = Vec::new();

    let mut preprecip_types: Vec<String> = Vec::new();
    preprecip_types.push(String::from(""));

    let mut sumarys: Vec<String> = Vec::new();
    sumarys.push(String::from(""));

    let mut icons: Vec<String> = Vec::new();
    icons.push(String::from(""));

    let mut h: Hour = new();

    let filename = "/home/shep/bin/data/hourly.csv";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut lastday: u32 = 0; //hr[0].time.day();
    let mut dayix = 0;
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            // first line skip text headers
            continue;
        }
        let tm = line
            .unwrap()
            .split(',')
            .map(str::to_owned)
            .collect::<Vec<_>>();

        let date = tm[0].to_string();
        h.time = NaiveDateTime::parse_from_str(&date, "%m/%d/%Y %H:%M:%S")
            .ok()
            .unwrap();
        // h.temperature = tm[1].trim().parse::<f32>().unwrap();
        if lastday != h.time.day() {
            lastday = h.time.day();
            day_break[ dayix ] = index - 1;
            dayix += 1;
        }
        h.temperature = tm[1].trim().parse::<f32>().unwrap();

        h.apparent_temperature = tm[2].trim().parse::<f32>().unwrap();
        h.dew_point = tm[3].trim().parse::<f32>().unwrap();
        h.humidity = tm[4].trim().parse::<f32>().unwrap();
        h.pressure = tm[5].trim().parse::<f32>().unwrap();
        h.precip_intensity = tm[6].trim().parse::<f32>().unwrap() * (1. / 25.4); // mm/hr => in/hr
        h.precip_probability = tm[7].trim().parse::<f32>().unwrap() * 100.;
        let t = tm[8].trim();
        h.precip_type = par_pre(&t, &mut preprecip_types);

        h.wind_speed = tm[9].trim().parse::<f32>().unwrap();
        h.wind_gust = tm[10].trim().parse::<f32>().unwrap();
        h.wind_bearing = tm[11].trim().parse::<f32>().unwrap();
        h.cloud_cover = tm[12].trim().parse::<f32>().unwrap();
        h.uv_ix = tm[13].trim().parse::<f32>().unwrap();
        h.visibility = tm[14].trim().parse::<f32>().unwrap();
        h.ozone = tm[15].trim().parse::<f32>().unwrap();
        let t = &tm[16];
        h.summary = par_sum(&t, &mut sumarys);
        let t = &tm[17];
        h.icon = par_icon(&t, &mut icons);
        hr.push(h);
    }
    day_break[3] = hr.len();
    daymm.push(Daymm{max:0,  min: 0});
    // daymm.push(Daymm{max:0,  min: 0});
    for el in 1..9{
        daymm.push( dmm(&hr, el, 0, 49));
    }

    let mut lastday: u32 = 0; //hr[0].time.day();

    for (index, i) in hr.iter().enumerate() {
        if i.time.day() != lastday {
            lastday = i.time.day();
            println!(
                "\x1b[4m{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}            \x1b[0m",
                " Time ",
                "  Tmp ",
                "  Atmp",
                " Hum",
                " Wind",
                "  Pres ",
                "  CC%",
                " PP%",
                " Pt        ",
                " in/hr",
                " Sum"
            );
        }
        print!("{:3}-{:02}|", i.time.weekday(), i.time.hour());
        if index == daymm[1].max{
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[1].min {
            print!("\x1b[31m"); // 'red': '\x1b[31m',
        }
        print!("{:6.2}", i.temperature);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        if index == daymm[2].max {
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[2].min {
            print!("\x1b[31m"); // 'red': '\x1b[31m',
        }
        print!("{:6.2}", i.apparent_temperature);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        if index == daymm[4].max{
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[4].min {
            print!("\x1b[31m"); // 'red' : '\x1b[31m',
        }
        print!("{:3.0}%", i.humidity);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        if index == daymm[8].max {
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[8].min {
            print!("\x1b[31m"); // 'red' : '\x1b[31m',
        }
        print!("{:5.1}", i.wind_speed,);
        print!("\x1b[0m|"); //  # actually black but whatevs

        if index == daymm[5].max {
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[5].min {
            print!("\x1b[31m"); // 'red' : '\x1b[31m',
        }
        print!(" {:5.1}", i.pressure,);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        if i.cloud_cover > 50. {
            print!("*{:3.0}%|", i.cloud_cover);
        } else {
            print!("{:4.0}%|", i.cloud_cover);
        }

        if index == daymm[7].max{
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[7].min {
            print!("\x1b[31m"); // 'red' : '\x1b[31m',
        }
        print!("{:3.0}%", i.precip_probability);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        print!(" {:10}| ", preprecip_types[i.precip_type]);
        if index == daymm[7].max {
            print!("\x1b[32m"); // 'green' : '\x1b[32m',
        } else if index == daymm[7].min {
            print!("\x1b[31m"); // 'red' : '\x1b[31m',
        }
        print!("{:3.3}", i.precip_intensity);
        print!("\x1b[0m\x1b[0m|"); //  # actually black but whatevs

        println!(" {:20}", sumarys[i.summary]);
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?} ", duration);
    // dbg!(day_break);
}

fn par_pre(needleraw: &str, sum: &mut Vec<String>) -> usize {
    let mut it = sum.iter();
    if needleraw.is_empty() {
        return 0; // frin ele
    }
    let n = needleraw.strip_prefix("\"").unwrap();
    let needle = n.strip_suffix("\"").unwrap().trim();

    let l = it.position(|r| r == needle);
    match l {
        Some(x) => {
            let index = x;
            // println!("ex{} {}", index, needle);
            index
        }
        None => {
            sum.push(String::from(needle));
            sum.len() - 1
        }
    }
}
fn par_sum(needleraw: &str, sum: &mut Vec<String>) -> usize {
    let mut it = sum.iter();
    if needleraw.is_empty() {
        return 0; // first is alway is "" ele
    }
    let n = needleraw.strip_prefix("\"").unwrap();
    let needle = n.strip_suffix("\"").unwrap().trim();

    let l = it.position(|r| r == needle);
    match l {
        Some(x) => {
            let index = x;
            // println!("ex{} {}", index, needle);
            index
        }
        None => {
            sum.push(String::from(needle));
            sum.len() - 1
        }
    }
}
fn par_icon(needleraw: &str, icons: &mut Vec<String>) -> usize {
    let mut it = icons.iter();
    let needle = needleraw;

    if !needleraw.is_empty() {
        let n = needleraw.strip_prefix("\"").unwrap();
        let _needle = n.strip_suffix("\"").unwrap();
    }

    let l = it.position(|r| r == needle);
    match l {
        Some(x) => {
            let index = x;
            // println!("ex{} {}", index, needle);
            index
        }
        None => {
            icons.push(String::from(needle));
            icons.len() - 1
        }
    }
}

fn dmm(h: &Vec<Hour>, el: usize, start: usize, end: usize) -> (Daymm) {
    let mut max: f32 = f32::MIN;
    let mut maxe: usize = 0;
    let mut min: f32 = f32::MAX;
    let mut mine: usize = 0;


    for  index in start..end {
        let t: f32 = hrindex(h[index], el);
        if t > max {
            maxe = index;
            max = t;
        }
        if t < min {
            mine = index;
            min = t;
        }
    }
    Daymm{max:maxe,  min: mine}
}
