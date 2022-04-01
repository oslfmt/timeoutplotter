use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::str;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::thread;
use std::process::{Command, Stdio};
// alternatively can use oping library or process
use ping::ping;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut addresses = vec![];

    for addr in &args[1..] {
        addresses.push(Ipv4Addr::from_str(&addr).unwrap());
    }

    measureSampleRTT(&addresses);
}

fn measureSampleRTT(targets: &[Ipv4Addr]) {
    // TODO: make loop 100 times
    for i in 0..2 {
        // every 5 seconds ping each target
        let sampleRTT: Vec<u128> = pingAll(targets);
        // store sampleRTT values
        
    }
}

fn pingAll(targets: &[Ipv4Addr]) -> Vec<u128> {
    let mut sampleRTTvalues: Vec<u128> = vec![];
    for addr in targets.to_vec() {
        // TODO: figure out how threads work
        // We'll need a way to differentiate each sampleRTT value, ie which
        // server it came from. We could probably use a HashMap<Ipv4Addr, Vec<u128>>, which
        // associates a series of pings with a particular target address.
        let sampleRTT = ping_time(addr.to_owned());
    }
    sampleRTTvalues
}

/// Sends out a ping to the address and returns the round-trip time
fn ping_time(address: Ipv4Addr) -> f32 {
    // send 100 ping packets, with 5s interval between each
    //
    // TODO: figure out what happens on failure to receive reply
    // could loop until 100 time values have been received in a vector, ie, vec.len() == 100
       
    let mut ping_child = Command::new("ping")
        //.arg("-i")
        //.arg("5")
        //.arg("-c")
        //.arg("1")
        .arg(address.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let ping_out = ping_child.wait_with_output().expect("Failed to open ping stdout");

    // have to a lot of conversions and parsing here
    let output_str = str::from_utf8(&ping_out.stdout).unwrap();

    let str_tokens: Vec<&str> = output_str.split(" ").collect();
    let elapsed_time = str_tokens[12];
    let split: Vec<&str> = elapsed_time.split("=").collect();

    let num_str = split[1];
    let num: f32 = num_str.parse().unwrap();
    num
}

fn measureEstimatedRTT() {
    // use sampleRTT values to find measure estimatedRTT
    // plot sampleRTT & estimatedRTT for each target, use plotters package
}

fn calculateTimeoutInterval() {
    // calculate for each target based on sample & estimatedRTT
    // plot timeoutInterval vs. time for each target
}

