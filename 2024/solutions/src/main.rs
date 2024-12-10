mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn check<T: std::cmp::PartialEq + std::fmt::Debug + std::fmt::Display>(res: T, expected: T) {
    assert_eq!(
        res, expected,
        "Test failed with value: {:?} != {}",
        res, expected
    );
}

fn main() {
    check(day1::solve_1(), 1319616);
    check(day1::solve_2(), 27267728);
    check(day2::solve_1(), 218);
    check(day2::solve_2(), 290);
    check(day3::solve_1(), 174561379);
    check(day3::solve_2(), 106921067);
    check(day4::solve_1(), 2401);
    check(day4::solve_2(), 1822);
    check(day5::solve_1(), 6949);
    check(day5::solve_2(), 4145);
    check(day6::solve_1(), 5516);
    check(day6::solve_2(), 2008);
    check(day7::solve_1(), 1430271835320);
    check(day7::solve_2(), 456565678667482);
}
