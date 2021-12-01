extern crate rayon;

// GENERATION 2D //
pub mod generation {
    pub mod two_d {
        use rayon::prelude::*;

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
            let mut table: Vec<Vec<u32>> = Vec::with_capacity(n.try_into().unwrap());
            (1..n+1).into_par_iter().map(|i| {
                let mut inner_table = Vec::with_capacity(n.try_into().unwrap());
                (1..n+1).into_par_iter().map(|j| {i*j}).collect_into_vec(&mut inner_table);
                inner_table
            }).collect_into_vec(&mut table);
            table
        }

        pub fn generate_multiplication_table_smart(n: u32) -> Vec<Vec<u32>> {
            if n < 280 {
                generate_multiplication_table(n)
            } else {
                generate_multiplication_table_async(n)
            }
        }

        pub fn generate_multiplication_table_split(n: u32) -> Vec<Vec<u32>> {
            let mut table = vec![vec![0; n.try_into().unwrap()]; n.try_into().unwrap()];
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
            let mut table = Vec::with_capacity((n*n).try_into().unwrap());
            for i in 1..n+1 {
                for j in 1..n+1 {
                    table.push(i*j);
                }
            }
            table
        }

        pub fn generate_multiplication_table_split(n: u32) -> Vec<u32> {
            let mut table = vec![0; (n*n).try_into().unwrap()];

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