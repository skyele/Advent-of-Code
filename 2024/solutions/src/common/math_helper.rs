pub fn get_num_digits(num: i64) -> i64 {
    (num as f64).log(10.0).floor() as i64 + 1
}
