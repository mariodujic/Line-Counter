use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;


pub fn show_work_in_progress_indicator() {

    const BACKSPACE: char = 8u8 as char;

    std::thread::spawn(move || {
        let mut state = 0;

        print!("Working...");
        loop {
            let symbol = match state {
                0 => "|",
                1 => "/",
                2 => "—",
                _ => "\\"
            };
            print!("{}{}", BACKSPACE, symbol);
            stdout().flush().unwrap();
            if state == 3 {
                state = 0;
            }
            state += 1;
            sleep(Duration::from_millis(150));
        }
    });
}