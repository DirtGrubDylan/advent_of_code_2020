mod expense_report;

use crate::util::file_reader::to_string_vector;

use expense_report::ExpenseReport;

pub fn run_day_1() {
    let expenses = to_string_vector("inputs/day_1.txt").unwrap();

    let expense_report = ExpenseReport::new(&expenses);

    let part_1 = expense_report.multiply_two_values_that_sum_to(2020);
    let part_2 = expense_report.multiply_three_values_that_sum_to(2020);

    println!("Day 1 Part 1: {}", part_1.unwrap());
    println!("Day 1 Part 2: {}", part_2.unwrap());
}
