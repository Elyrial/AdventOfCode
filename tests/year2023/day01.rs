use advent_of_code::solutions::year2023::day01::Day01;

#[test]
fn test_part1() {
    assert_eq!(Day01.part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), "142");
}

fn test_part2() {
    assert_eq!(Day01.part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), "281");
}
