#![allow(dead_code)]
//! This module contains the code for the multiplication table.
//! Its purpose is to experiment with different ways of optimizing a simple function and learning rust.
extern crate rayon;

pub mod generation {
    pub mod two_d {
        use std::cell::RefCell;
        use rayon::prelude::*;

        /// Generates a multiplication table with sides of `n` length using a simple iterative approach.
        pub fn generate_multiplication_table(n: u32) -> Vec<Vec<u32>> {
            let mut table = Vec::with_capacity(n as usize);
            for i in 1..n+1 {
                let mut row = Vec::with_capacity(n as usize);
                for j in 1..n+1 {
                    row.push(i*j);
                }
                table.push(row);
            }
            table
        }
        thread_local!(
            /// The cache for the `generate_multiplication_table_cached` function.
            static CACHED_TABLE: RefCell<Vec<Vec<u32>>> = RefCell::new(Vec::new())
        );
        /// Generates a multiplication table with sides of `n` length using a unique in-memory cacheing approach.
        pub fn generate_multiplication_table_cached(n: u32) -> Vec<Vec<u32>> {
            CACHED_TABLE.try_with(|cache| {
                if cache.borrow().len() == n as usize { // copy
                    cache.borrow().clone()
                } else if cache.borrow().len() < n as usize { // grow the cache
                    let new_table = generate_multiplication_table_smart(n);
                    if cache.borrow().is_empty() {
                        cache.borrow_mut().extend(new_table.clone());
                        new_table
                    } else {
                        // extend existing rows
                        for (i, row) in cache.borrow_mut().iter_mut().enumerate() { 
                            row.append(new_table[i].clone().split_off(row.len()).as_mut() );
                        }
                        // append new rows
                        cache.borrow_mut().append(new_table.clone().as_mut());
                        
                        new_table
                    }
                } else { // copy and truncate
                    let mut new_table: Vec<Vec<u32>> = cache.borrow().clone();
                    new_table.truncate(n as usize);
                    for row in new_table.iter_mut() {
                        row.truncate(n as usize);
                    }
                    new_table
                }
            }).unwrap_or_else(|_|generate_multiplication_table_smart(n))
        }

        /// Generates a multiplication table with sides of `n` length using the rayon crate for parallelism.
        pub fn generate_multiplication_table_par(n: u32) -> Vec<Vec<u32>> {
            let mut table: Vec<Vec<u32>> = Vec::with_capacity(n as usize);
            (1..n+1).into_par_iter().map(|i| {
                let mut inner_table = Vec::with_capacity(n as usize);
                (1..n+1).into_par_iter().map(|j| {i*j}).collect_into_vec(&mut inner_table);
                inner_table
            }).collect_into_vec(&mut table);
            table
        }
        /// The cutoff point for the `generate_multiplication_table_smart` function where it switches to using parallelism.
        pub const SMART_CUTOFF: u32 = 625;
        /// Generates a multiplication table with sides of `n` length by using a simple iterative approach for `n < SMARTCUTTOFF` and a parallell approach when greater.
        pub fn generate_multiplication_table_smart(n: u32) -> Vec<Vec<u32>> {
            if n < SMART_CUTOFF {
                generate_multiplication_table(n)
            } else {
                generate_multiplication_table_par(n)
            }
        }

        pub fn generate_multiplication_table_split(n: u32) -> Vec<Vec<u32>> {
            let mut table = vec![vec![0; n as usize]; n as usize];
            let mut i = 0;
            let mut j = 0;
            while i < n {
                let res = (i+1)*(j+1);
                table[i as usize][j as usize] = res;
                table[j as usize][i as usize] = res;
                j += 1;
                if j > i {
                    i += 1;
                    j = 0;
                }
            }
            table
        }
    }
    pub mod one_d {

        pub fn generate_multiplication_table(n: u32) -> Vec<u32> {
            let mut table = Vec::with_capacity(n as usize * n as usize);
            for i in 1..n+1 {
                for j in 1..n+1 {
                    table.push(i*j);
                }
            }
            table
        }

        pub fn generate_multiplication_table_split(n: u32) -> Vec<u32> {
            let mut table = vec![0; n as usize * n as usize];

            let mut i = 0;
            let mut j = 0;

            while i < n {
                let res = (i+1)*(j+1);
                table[(i*n+j) as usize] = res;
                table[(j*n+i) as usize] = res;
                j += 1;
                if j > i {
                    i += 1;
                    j = 0;
                }
            }

            table
        }
    }
}
pub mod printing {
    pub fn print_multiplication_table_1d(table: &Vec<u32>) {
        let n = f64::sqrt(table.len() as f64).floor() as u32;

        for i in 0..table.len() {
            if (i) as u32 % (n) == 0 {
                println!("");
            }
            print!("{:>4}", table[i]);
        }
        println!("");
    }

    pub fn print_multiplication_table(table: &Vec<Vec<u32>>) {
        for row in table {
            for col in row {
                print!("{:>4}", col);
            }
            println!();
        }
    }
}



#[cfg(test)]
mod test {
    const N: u32 = 9;
    const TABLE_9: [u32; 81] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9,
        2, 4, 6, 8, 10, 12, 14, 16, 18,
        3, 6, 9, 12, 15, 18, 21, 24, 27,
        4, 8, 12, 16, 20, 24, 28, 32, 36,
        5, 10, 15, 20, 25, 30, 35, 40, 45,
        6, 12, 18, 24, 30, 36, 42, 48, 54,
        7, 14, 21, 28, 35, 42, 49, 56, 63,
        8, 16, 24, 32, 40, 48, 56, 64, 72,
        9, 18, 27, 36, 45, 54, 63, 72, 81
    ];

    mod two_d {
        #[allow(unused_imports)]
        use crate::multiplication_table::generation::two_d::*;
        use super::{N, TABLE_9};

        fn test_helper_generate_table(generator: fn(u32) -> Vec<Vec<u32>>) {
            let table = generator(N);
            // test length of columns and rows
            assert_eq!(table.len(), N as usize);
            for row in &table {
                assert_eq!(row.len(), N as usize);
            }
            // test table contents
            for (i, num) in TABLE_9.iter().enumerate() {
                assert_eq!(table[i/N as usize][i%N as usize], *num);
            } 
        }
        #[test]
        fn test_generate_multiplication_table_par() {
            test_helper_generate_table(generate_multiplication_table_par);
        }
        #[test]
        fn test_generate_multiplication_table_smart() {
            test_helper_generate_table(generate_multiplication_table_smart);
        }
        #[test]
        fn test_generate_multiplication_table_cached() {
            test_helper_generate_table(generate_multiplication_table_cached);
            test_helper_generate_table(generate_multiplication_table_cached);
        }
        #[test]
        fn test_generate_multiplication_table_split() {
            test_helper_generate_table(generate_multiplication_table_split);
        }
    }
    mod one_d {

    }

}