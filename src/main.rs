use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut addresses = vec![];

    for addr in args {
        addresses.push(Ipv4Addr::from_str(&addr).unwrap());
    }
}

fn measureSampleRTT(targets: Vec<Ipv4Addr>) {
    for i in 0..99 {
        // every 5 seconds ping each target
        // store sampleRTT value
    }
}

fn measureEstimatedRTT() {
    // use sampleRTT values to find measure estimatedRTT
    // plot sampleRTT & estimatedRTT for each target, use plotters package
}

fn calculateTimeoutInterval() {
    // calculate for each target based on sample & estimatedRTT
    // plot timeoutInterval vs. time for each target
}

