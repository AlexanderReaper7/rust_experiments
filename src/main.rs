extern crate rayon;
use rayon::prelude::*;

pub fn main() {
    let table = generate_multiplication_table(30);
    print_multiplication_table(&table);
}

pub fn generate_multiplication_table(n: u32) -> Vec<Vec<u32>> {
    let mut table = Vec::with_capacity(n.try_into().unwrap());
    for i in 1..n+1 {
        let mut row = Vec::with_capacity(n.try_into().unwrap());
        for j in 1..n+1 {
            row.push(i*j);
        }
        table.push(row);
    }
    table
}

pub fn generate_multiplication_table_async(n: u32) -> Vec<Vec<u32>> {
    let mut table = (1..n+1).into_par_iter().map(|i| {
        (1..n+1).into_par_iter().map(|j| {i*j}).collect()
    }).collect();
    table
}

pub fn generate_multiplication_table_smart(n: u32) -> Vec<Vec<u32>> {
    if n < 100 {
        generate_multiplication_table(n)
    } else if n < 8192 {
        generate_multiplication_table_async(n)
    } else {
        generate_multiplication_table(n)
    }
}


pub fn print_multiplication_table(table: &Vec<Vec<u32>>) {
    for row in table {
        for col in row {
            print!("{:>4}", col);
        }
        println!();
    }
}