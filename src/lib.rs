use std::fs;
use std::env;
use std::fmt::Display;
use std::time::Instant;
pub fn read_input_file(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("src").join("inputs").join(format!("day{:02}.txt", day));

    let content = fs::read_to_string(path);

    content.expect("couldn't read file")
}

pub fn solve<T: Display>(part: u8, solver: impl FnOnce(&String) -> Option<T>, input: &String) {
    let timer = Instant::now();
    let result = solver(input);
    let time_elapsed = timer.elapsed();


    println!("Part {}:", part);
    match result {
        Some(result) => {
            println!("{} (elapsed: {:.2?})", result, time_elapsed);
        }
        None => {
            println!("not solved");
        }
    }
}
