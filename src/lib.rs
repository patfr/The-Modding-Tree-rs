use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/*
use std::{
    thread,
    time::{Duration},
};
*/

#[wasm_bindgen]
pub fn main() {
    log("Hello")
    //alert("str");
}

/*
#[wasm_bindgen]
pub fn main() {
    loop {
        println!("custom");
        thread::sleep(Duration::from_millis(1000));
    }

    /*
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_millis(500);

        for _ in 0..5 {
            let start = Instant::now();
            eprintln!("Scheduler starting at {:?}", start);

            let thread_a = thread::spawn(a);
            let thread_b = thread::spawn(b);

            thread_a.join().expect("Thread A panicked");
            thread_b.join().expect("Thread B panicked");

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                eprintln!(
                    "schedule slice has time left over; sleeping for {:?}",
                    remaining
                );
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().expect("Scheduler panicked");
    */
}
*/