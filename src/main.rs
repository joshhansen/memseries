use std::{
    ffi::OsStr,
    process::{Child, Command, Stdio},
    time::{Duration, SystemTime},
};

use byte_unit::{Byte, Unit};
use clap::Parser;
use procinfo::pid::statm;

fn parse_multiplier(m: char) -> u64 {
    if m == 'm' {
        return 60;
    }

    if m == 'h' {
        return 3600;
    }

    if m == 'd' {
        return 3600 * 24;
    }

    return 1;
}

fn parse_duration(s: &str) -> u64 {
    let multiplier = parse_multiplier(s.chars().last().unwrap());

    let value: u64 = if multiplier > 1 {
        s[0..s.len() - 1].parse().unwrap()
    } else {
        s.parse().unwrap()
    };

    value * multiplier
}

/// Generates timeseries of program RSS usage
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The program whose RSS to track
    #[arg()]
    cmd: String,

    /// Number of times to run
    #[arg(short, default_value_t = 5)]
    n: usize,

    /// How long each run lasts, in seconds; append m for minutes, h for hours, d for days
    #[arg(short, long, default_value = "2h")]
    duration: String,

    /// Sampling interval in milliseconds; how often RSS is measured and recorded
    #[arg(short, long = "sample", default_value_t = 60000)]
    sampling_interval: u64,

    /// How many runs to perform as burn-in before tracking
    #[arg(short, long = "burn", default_value_t = 1)]
    burn_in: usize,

    /// Units to report results in; as parsed by byte_unit::Unit::parse_str
    #[arg(short, long, default_value = "KB")]
    units: Unit,
}

/// RSS memory usage in bytes
fn rss(pid: u32) -> usize {
    let statm = statm(pid as i32).unwrap();

    let pages = statm.resident;

    let page_size = page_size::get();

    pages * page_size
}

fn spawn<S: AsRef<OsStr>>(s: S) -> Child {
    Command::new(s).stdout(Stdio::null()).spawn().unwrap()
}

fn end_process(p: &mut Child) {
    p.kill().unwrap();
    p.wait().unwrap();
}

const SEP: char = '\t';
fn main() {
    let args = Args::parse();

    let duration = parse_duration(args.duration.as_str());

    println!(
        "Running {} {} times for {}s; burn-in of {} times; sampling every {}ms",
        args.cmd, args.n, duration, args.burn_in, args.sampling_interval
    );

    let duration = Duration::from_secs(duration);

    let interval = Duration::from_millis(args.sampling_interval);

    for i in 0..args.burn_in {
        eprintln!("Burn-in {}", i);

        let mut p = spawn(&args.cmd);

        std::thread::sleep(duration);

        end_process(&mut p);

        eprintln!("Burn-in {} done", i);
    }

    // RSS memory usage in bytes
    // [run, sample]
    let mut samples: Vec<Vec<usize>> = Vec::with_capacity(args.n);
    for i in 0..args.n {
        eprintln!("Run {}", i);
        let start = SystemTime::now();
        let last_sample: Option<SystemTime> = None;
        let p = spawn(&args.cmd);

        let pid = p.id();

        let mut run_samples: Vec<usize> = Vec::new();

        'inner: loop {
            let sample = if let Some(last_sample) = &last_sample {
                let since_last_sample = last_sample.elapsed().unwrap();

                since_last_sample >= interval
            } else {
                true
            };

            if sample {
                run_samples.push(rss(pid));
            }

            if start.elapsed().unwrap() > duration {
                let mut p = p;

                end_process(&mut p);

                samples.push(run_samples);
                eprintln!("Run {} done", i);
                break 'inner;
            }

            std::thread::sleep(interval);
        }
    }

    let max_collected = samples.iter().map(|s| s.len()).max().unwrap();

    print!("mean{}stdev", SEP);
    for i in 0..args.n {
        print!("{}", SEP);
        print!("{}", i);
    }
    println!();

    for j in 0..max_collected {
        let total: usize = (0..args.n).map(|i| samples[i][j]).sum();
        let mean = total as f64 / args.n as f64;

        let stdev = {
            let mut sum_squares = 0f64;
            for x in (0..args.n).map(|i| samples[i][j]) {
                sum_squares += (x as f64) * (x as f64);
            }

            let expected_sum_squares = sum_squares / args.n as f64;

            f64::sqrt(expected_sum_squares - mean * mean)
        };

        let mean: Byte = (mean as usize).into();
        let stdev: Byte = (stdev as usize).into();

        print!("{}{}{}", mean, SEP, stdev);
        for i in 0..args.n {
            print!("{}", SEP);
            let value = samples[i]
                .get(j)
                .map(|v| {
                    let v: Byte = (*v).into();

                    let v = v.get_adjusted_unit(args.units);

                    v.to_string()
                })
                .unwrap_or_default();
            print!("{}", value);
        }
        println!();
    }
}
