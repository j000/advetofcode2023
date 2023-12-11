use rstest::rstest;

use advetofcode2023::day1;

#[rstest]
#[case('a', false)]
#[case('t', false)]
#[case('z', false)]
#[case('A', false)]
#[case('T', false)]
#[case('Z', false)]
#[case('0', true)]
#[case('9', true)]
#[case('1', true)]
#[case('7', true)]
fn should_recognize_numbers(#[case] input: char, #[case] expected: bool) {
    assert_eq!(expected, day1::is_number(&input));
}

#[rstest]
#[case("test", None)]
#[case("1abc2", Some(12))]
#[case("pqr3stu8vwx", Some(38))]
#[case("a1b2c3d4e5f", Some(15))]
#[case("treb7uchet", Some(77))]
fn should_find_numbers(#[case] input: &str, #[case] expected: Option<i64>) {
    assert_eq!(expected, day1::find_number(input));
}
