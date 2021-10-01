use core::time::Duration;
use std::{collections::BTreeMap, thread::sleep};

fn main() {
    let mut meter = self_meter::Meter::new(Duration::new(1, 0)).unwrap();
    meter.track_current_thread("main");

    let mut v = vec![0u64; 0];

    loop {
        meter.scan()
            .map_err(|e| eprintln!("Scan error: {}", e)).ok();
        println!("Report: {:#?}", meter.report());
        println!("Threads: {:#?}",
            meter.thread_report().map(|x| x.collect::<BTreeMap<_,_>>()));
        // Put your task here
        // ...
        //
        v.append(&mut vec![0u64; 125000]); // +1 Mb

        sleep(Duration::new(1, 0));
    }
 }