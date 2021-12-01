mod multiplication_table;

use crate::multiplication_table::{generation::two_d::generate_multiplication_table_smart, printing::print_multiplication_table};

pub fn main() {
    // multiplication table
    let n = 25u32;

    let table = generate_multiplication_table_smart(n);
    print_multiplication_table(&table);

}
