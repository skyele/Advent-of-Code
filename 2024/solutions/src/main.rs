mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    check(day8::solve_1(), 323);
    check(day8::solve_2(), 1077);
    check(day9::solve_1(), 6346871685398);
    check(day9::solve_2(), 6373055193464);
    check(day10::solve_1(), 468);
    check(day10::solve_2(), 966);
    check(day11::solve_1(), 231278);
    check(day11::solve_2(), 274229228071551);
    check(day12::solve_1(), 1549354);
    check(day12::solve_2(), 937032);
    check(day13::solve_1(), 37901);
    check(day13::solve_2(), 77407675412647);
}
