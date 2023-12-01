pub fn part1_solution(input: String) -> u32 {
    let mut vec: Vec<u32> = vec![];
    for line in into_lines(input) {
        let value = get_calibration_value_for_line(&line);
        vec.push(value);
    }
    vec.into_iter().sum()
}

fn into_lines(input: String) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect::<Vec<_>>()
}

fn get_calibration_value_for_line(line: &str) -> u32 {
    let mut solution = 0;
    let numbers = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();
    let first = numbers.get(0).unwrap() * 10;
    let second = numbers.get(numbers.len() - 1).unwrap();
    solution += first + second;
    solution
}

/* PART 2 SOLUTION */
pub fn part2_solution(input: String) -> u32 {
    let mut vec: Vec<u32> = vec![];
    for line in into_lines(input) {
        let value = get_calibration_value_for_line_part_2(&line);
        vec.push(value);
    }
    vec.into_iter().sum()
}

enum DigitVal {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl DigitVal {
    pub fn str_vals() -> Vec<&'static str> {
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
    }

    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "one" => Some(Self::ONE),
            "two" => Some(Self::TWO),
            "three" => Some(Self::THREE),
            "four" => Some(Self::FOUR),
            "five" => Some(Self::FIVE),
            "six" => Some(Self::SIX),
            "seven" => Some(Self::SEVEN),
            "eight" => Some(Self::EIGHT),
            "nine" => Some(Self::NINE),
            _ => None,
        }
    }

    pub fn to_digit(&self) -> u32 {
        match self {
            DigitVal::ONE => 1,
            DigitVal::TWO => 2,
            DigitVal::THREE => 3,
            DigitVal::FOUR => 4,
            DigitVal::FIVE => 5,
            DigitVal::SIX => 6,
            DigitVal::SEVEN => 7,
            DigitVal::EIGHT => 8,
            DigitVal::NINE => 9,
        }
    }
}

fn get_words_with_indices(line: &str) -> Vec<(usize, &str)> {
    let mut words: Vec<Vec<(usize, &str)>> = vec![];
    for val in DigitVal::str_vals() {
        let words_to_push: Vec<_> = line.match_indices(val).collect();
        words.push(words_to_push);
    }
    words.concat()
}

fn get_word_minmax(
    words_with_indices: Vec<(usize, &str)>,
) -> (Option<(usize, &str)>, Option<(usize, &str)>) {
    let min = words_with_indices.iter().min();
    let max = words_with_indices.iter().max();
    match (min, max) {
        (None, None) => (None, None),
        (None, Some(v)) => (None, Some(*v)),
        (Some(v), None) => (Some(*v), None),
        (Some(v), Some(z)) => (Some(*v), Some(*z)),
    }
}

fn get_num_minmax(
    nums_with_indices: Vec<(usize, u32)>,
) -> (Option<(usize, u32)>, Option<(usize, u32)>) {
    let min = nums_with_indices.iter().min();
    let max = nums_with_indices.iter().max();
    match (min, max) {
        (None, None) => (None, None),
        (None, Some(v)) => (None, Some(*v)),
        (Some(v), None) => (Some(*v), None),
        (Some(v), Some(z)) => (Some(*v), Some(*z)),
    }
}

fn get_min(word_minmax: &Option<(usize, &str)>, num_minmax: &Option<(usize, u32)>) -> u32 {
    match (word_minmax, num_minmax) {
        (None, None) => 0,
        (None, Some(v)) => v.1,
        (Some(v), None) => DigitVal::from_str(v.1).unwrap().to_digit(),
        (Some(word_min), Some(num_min)) => match word_min.0 > num_min.0 {
            true => num_min.1,
            false => DigitVal::from_str(word_min.1).unwrap().to_digit(),
        },
    }
}

fn get_max(word_minmax: &Option<(usize, &str)>, num_minmax: &Option<(usize, u32)>) -> u32 {
    match (word_minmax, num_minmax) {
        (None, None) => 0,
        (None, Some(v)) => v.1,
        (Some(v), None) => DigitVal::from_str(v.1).unwrap().to_digit(),
        (Some(word_min), Some(num_min)) => match word_min.0 > num_min.0 {
            false => num_min.1,
            true => DigitVal::from_str(word_min.1).unwrap().to_digit(),
        },
    }
}

fn get_calibration_value_for_line_part_2(line: &str) -> u32 {
    let numbers_with_indices = line
        .chars()
        .enumerate()
        .filter_map(|(idx, char)| char.to_digit(10).and_then(|x| Some((idx, x))))
        .collect::<Vec<_>>();
    let words_with_indices = get_words_with_indices(line);
    let word_minmax = get_word_minmax(words_with_indices);
    let num_minmax = get_num_minmax(numbers_with_indices);
    let min = get_min(&word_minmax.0, &num_minmax.0);
    let max = get_max(&word_minmax.1, &num_minmax.1);
    (min * 10) + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_solution1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1_solution(input.to_string()), 142);
    }

    #[test]
    fn get_words_works() {
        let input = "two1nine";
        assert_eq!(get_words_with_indices(input), vec![(0, "two"), (4, "nine")]);
    }

    #[test]
    fn get_word_minmax_test() {
        assert_eq!(
            get_word_minmax(vec![(0, "two"), (4, "nine")],),
            (Some((0, "two")), Some((4, "nine")))
        );
    }

    #[test]
    fn get_min_works() {
        let num = (Some((0, 4)), Some((15, 2)));
        let str = (Some((1, "nine")), Some((10, "seven")));
        assert_eq!(get_min(&str.0, &num.0), 4);
    }

    #[test]
    fn get_max_works() {
        let num = (Some((0, 4)), Some((15, 2)));
        let str = (Some((1, "nine")), Some((10, "seven")));
        assert_eq!(get_max(&str.1, &num.1), 2);
    }

    #[test]
    fn calibration_2() {
        let input = "treb7uchet";
        assert_eq!(get_calibration_value_for_line_part_2(input), 77);
    }

    #[test]
    fn part2_solution1() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2_solution(input.to_string()), 281);
    }
}
