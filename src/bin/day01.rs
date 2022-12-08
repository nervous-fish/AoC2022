#![feature(test)]
extern crate test;
use test::Bencher;


fn main() {
    let input: String = aoc_2022::read_input_file(1);

    aoc_2022::solve(1, part_one, &input);

    aoc_2022::solve(2, part_two, &input);
}

fn part_one(input: &String) -> Option<u32> {
    get_calories_per_elf(input).iter().max().copied()
}

fn part_two(input: &String) -> Option<u32> {
    let mut calories_per_elf = get_calories_per_elf(input);
    calories_per_elf.sort(); 
    
    let top3 = calories_per_elf.iter().rev().take(3);

    Some(top3.sum::<u32>())
}

fn get_calories_per_elf(input: &String) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .iter()
        .fold(vec![0], |mut acc, value| match value.parse::<u32>() {
            Ok(parsed_value) => {
                let current_total = acc.pop().unwrap();
                acc.push(current_total + parsed_value);
                acc
            }
            Err(_) => {
                acc.push(0);
                acc
            }
        })
}

#[bench]
fn bench_part_one(b: &mut Bencher) {
    let input: String = test::black_box(aoc_2022::read_input_file(1));

    b.iter(|| {
        part_one(&input)
    });
}

#[bench]
fn bench_part_two(b: &mut Bencher) {
    let input: String = test::black_box(aoc_2022::read_input_file(1));

    b.iter(|| {
       part_two(&input);
    });
}