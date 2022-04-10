use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::str;
use std::time::{Duration, Instant};
use std::thread;
use std::process::{Command, Stdio, Output};
use std::collections::HashMap;
use std::sync::mpsc;

use gnuplot::{Figure, Caption, Color};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut addresses = vec![];

    for addr in &args[1..] {
        addresses.push(Ipv4Addr::from_str(&addr).unwrap());
    }

    let sample_rtts_map = ping_all(&addresses);
    plot(sample_rtts_map);
}

fn plot(data_map: HashMap<Ipv4Addr, Vec<(f32, (f32, (f32, f32)))>>) {
    for (address, values) in data_map.iter() {
        let path = String::from("/home/victor/project2/graphs/") + &address.to_string();
        let path_to = String::from("/home/victor/project2/graphs/") + &address.to_string() + "timeout";
        let mut fg = Figure::new();
        let mut fg_to = Figure::new();

        let (times, (sample_rtts, (est_rtts, timeout_intervals))): (Vec<f32>, (Vec<f32>, (Vec<f32>, Vec<f32>)))
                                                                    = values.iter().cloned().unzip();

        fg.set_title(&address.to_string())
            .axes2d()
            .lines(&times, sample_rtts, &[Caption("SampleRTTs"), Color("black")])
            .lines(&times, est_rtts, &[Caption("EstimatedRTTs"), Color("red")]);

        fg_to.set_title(&address.to_string())
            .axes2d()
            .lines(&times, timeout_intervals, &[Caption("Timeouts"), Color("blue")]);

        match fg.save_to_png(path, 1024, 1024) {
            Ok(()) => println!("{}", "Saved file"),
            Err(e) => println!("{}", e),
        };

        match fg_to.save_to_png(path_to, 1024, 1024) {
            Ok(()) => println!("{}", "Saved file"),
            Err(e) => println!("{}", e),
        };
    }
}

fn ping_all(targets: &[Ipv4Addr]) -> HashMap<Ipv4Addr, Vec<(f32, (f32, (f32, f32)))>> {
    let mut sample_rtts_map = HashMap::new();
    let (tx, rx) = mpsc::channel();

    for addr in targets.to_vec() {
        let mut rtt_values: Vec<(f32, (f32, (f32, f32)))> = vec![];
        let tx = tx.clone();

        thread::spawn(move || {
            let mut estimated_rtt = 0.0;
            let mut dev_rtt = 0.0;
            let mut to_interval;
            let now = Instant::now();

            while rtt_values.len() < 100 {
                match ping(addr) {
                    Ok(sample_rtt) => {
                        if rtt_values.len() == 0 {
                            estimated_rtt = sample_rtt;
                            let diff = sample_rtt - estimated_rtt;
                            dev_rtt = 0.25*diff.abs();
                        } else {
                            estimated_rtt = 0.875*estimated_rtt + 0.125*sample_rtt;
                            let diff = sample_rtt - estimated_rtt;
                            dev_rtt = (1.0-0.25)*dev_rtt + 0.25*diff.abs();
                        }

                        to_interval = estimated_rtt + 4.0*dev_rtt;
                        rtt_values.push((now.elapsed().as_millis() as f32, (sample_rtt, (estimated_rtt, to_interval))));

                        println!("{:?}: {}", addr, rtt_values.len());
                    },
                    Err(e) => println!("{}", e),
                };
                thread::sleep(Duration::from_millis(5000));
            }

            tx.send((addr, rtt_values)).unwrap();
        });
    }

    drop(tx);
    while let Ok((addr, rtt_values)) = rx.recv() {
        sample_rtts_map.insert(addr, rtt_values);
    }

    sample_rtts_map
}

/// Sends out a ping to the address and returns the round-trip time
fn ping(address: Ipv4Addr) -> Result<f32, String> {
    let ping_child = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(address.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let ping_out = ping_child.wait_with_output().expect("Failed to open ping stdout");

    let rtt = extract_time_from_output(ping_out)?;
    Ok(rtt)
}

fn extract_time_from_output(output: Output) -> Result<f32, String> {
    let output_str = str::from_utf8(&output.stdout).unwrap();
    let str_tokens: Vec<&str> = output_str.split(" ").collect();

    let elapsed_time = str_tokens[12];
    let split: Vec<&str> = elapsed_time.split("=").collect();
    //println!("{:?}", split);

    if split.len() == 2 {
        let num_str = split[1];
        Ok(num_str.parse().unwrap())
    } else {
        Err("Ping error".to_string())
    }
}

