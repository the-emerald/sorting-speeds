const ARRAY_SIZE_START: usize = 1000;
const ARRAY_SIZE_END: usize = 20_000;
const ARRAY_STEP_SIZE: usize = 1000;

const RUNS_PER_SIZE: usize = 1000;
//const REPEAT_TOTAL_RUNS: usize = 1;

mod mergesort;
mod writer;
use std::env;
use crate::writer::Writer;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("No files specified")
    }

    let mut writer = match Writer::new(&args[1], &args[2]) {
        Ok(x) => x,
        Err(_) => panic!("lol IO error u suck")
    };

    for i in (ARRAY_SIZE_START..ARRAY_SIZE_END + ARRAY_STEP_SIZE)
        .step_by(ARRAY_STEP_SIZE) {

        let gen_start = Instant::now();
        let mut arr: Vec<i32> = (1..=(i as i32)).collect();
        arr.shuffle(&mut thread_rng());
        let gen_duration = gen_start.elapsed();

        match writer.write_gen(i, gen_duration) {
            Ok(x) => x,
            Err(_) => panic!("Unable to write to file.")
        };

        let start = Instant::now();
        for _j in 0..RUNS_PER_SIZE {
            let mut arr_c = arr.to_vec();

            let sort_start = Instant::now();
            mergesort::mergesort(&mut arr_c);
            let sort_duration = sort_start.elapsed();

            match writer.write_run(&"merge", i, sort_duration) {
                Ok(x) => x,
                Err(_) => panic!("Unable to write to file.")
            };
        }
        let duration = start.elapsed();
        println!("Sorted array of size {} with {} trials: : {:?}", i, RUNS_PER_SIZE, duration);
    }
}
