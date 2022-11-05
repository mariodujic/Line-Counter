#[macro_use]
extern crate prettytable;

use std::env;

use crate::console_utils::show_work_in_progress_indicator;
use crate::file_utils::retrieve_line_count_per_file;

mod file_utils;
mod console_utils;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);

    if path.is_none() {
        panic!("Please pass a directory path as an argument.");
    }

    show_work_in_progress_indicator();

    let line_count = retrieve_line_count_per_file(path.unwrap());
    let mut table = table!(["Extension", "LOC"]);
    for file_type in line_count {
        table.add_row(row![file_type.0, file_type.1]);
    }
    println!("\n");
    table.printstd();
}