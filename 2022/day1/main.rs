/*
* The answer for the https://adventofcode.com/2022/day/1
* base on Andrew Burgess video on youtube!
*/

use std::fs;

fn main() {
  let content = fs::read_to_string("input.txt").expect("Can't load file!");

  let counts = content
    .split("\n\n")
    .map(|chunk| -> usize { chunk.split("\n").map(|row| row.parse().unwrap_or(0)).sum() });

  let mut v: Vec<_> = counts.collect::<Vec<_>>();

  v.sort();

  let last_idx = v.len() - 1;

  println!("the highest count is {}", v[last_idx]);
}