mod common;
mod day1_1;
mod day1_2;
mod day2;
mod day3;
mod day4;

fn check_day1_1() {
    let expected_value = 1319616;
    let day1_1_res = day1_1::solve();
    assert_eq!(
        day1_1_res, expected_value,
        "Test failed with value: {:?} != {}",
        day1_1_res, expected_value
    );
}

fn check_day1_2() {
    let expected_value = 27267728;
    let day1_2_res = day1_2::solve();
    assert_eq!(
        day1_2_res, expected_value,
        "Test failed with value: {:?} != {}",
        day1_2_res, expected_value
    );
}

fn check(res: i32, expected: i32) {
    assert_eq!(
        res, expected,
        "Test failed with value: {:?} != {}",
        res, expected
    );
}

fn main() {
    check(day1_1::solve(), 1319616);
    check(day1_2::solve(), 27267728);
    check(day2::solve_1(), 218);
    check(day2::solve_2(), 290);
    check(day3::solve_1(), 174561379);
    check(day3::solve_2(), 106921067);
    check(day4::solve_1(), 2401);
    check(day4::solve_2(), 1822);
}
