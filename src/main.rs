#![allow(dead_code)] // I want to be able to comment out test cases without warnings
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("--- Day 1 ---");
    day1::day_1();

    println!("--- Day 2 ---");
    day2::day_2();

    println!("--- Day 3 ---");
    day3::day_3();

    println!("--- Day 4 ---");
    day4::day_4();

    println!("--- Day 5 ---");
    day5::day_5();

    println!("--- Day 6 ---");
    day6::day_6();

    println!("--- Day 7 ---");
    day7::day_7();

    println!("--- Day 8 ---");
    day8::day_8();

    println!("--- Day 9 ---");
    day9::day_9();
}
