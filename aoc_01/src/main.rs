use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut combined_values: Vec<u32> = Vec::new();

    for line in &lines {
        let digits = get_digits_from_line(line);

        if digits.len() >= 2 {
            let combined = format!("{}{}", digits[0], digits[digits.len() - 1]);
            let combined = combined.parse::<u32>().unwrap();
            combined_values.push(combined);
        } else if digits.len() == 1 {
            let combined = format!("{}{}", digits[0], digits[0]);
            let combined = combined.parse::<u32>().unwrap();
            combined_values.push(combined);
        }
    }

    let mut sum: u32 = 0;
    for value in &combined_values {
        sum += *value;
    }

    println!("Sum: {:?}", sum);
}

fn get_digits_from_line(line: &str) -> Vec<u32> {
    let line_but_vec = &mut Vec::new();
    let mut line_but_vec_to_str: String = String::new();

    line_but_vec.extend(get_all_substring_vals(line));
    line_but_vec.extend(get_all_raw_nums(line));

    let sorted_vec: Vec<(usize, u32)> = sort_vec_by_index(line_but_vec.to_vec());
    line_but_vec_to_str.push_str(sorted_vec_to_str(&sorted_vec).as_str());

    let digits: Vec<u32> = line_but_vec_to_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    digits
}

fn get_all_substring_vals(line: &str) -> Vec<(usize, u32)> {
    let mut combined_substr_to_val_results = Vec::new();

    combined_substr_to_val_results.extend(find_all_of_substring(line, "one", 1));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "two", 2));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "three", 3));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "four", 4));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "five", 5));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "six", 6));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "seven", 7));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "eight", 8));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "nine", 9));
    combined_substr_to_val_results.extend(find_all_of_substring(line, "zero", 0));
    combined_substr_to_val_results
}

fn find_all_of_substring(main_str: &str, substring: &str, num_val: u32) -> Vec<(usize, u32)> {
    let mut results = Vec::new();
    let mut current_index = 0;

    while let Some(found_index) = main_str[current_index..].find(substring) {
        let real_index = current_index + found_index;
        results.push((real_index, num_val));
        current_index = real_index + substring.len();
    }
    results
}

fn get_all_raw_nums(line: &str) -> Vec<(usize, u32)> {
    let mut results = Vec::new();
    let mut current_index = 0;

    while let Some(found_index) = line[current_index..].find(|c: char| c.is_digit(10)) {
        let real_index = current_index + found_index;
        let mut current_num = String::new();
        let mut current_num_index = real_index;
        while let Some(char) = line[current_num_index..].chars().next() {
            if char.is_digit(10) {
                current_num.push(char);
                current_num_index += 1;
            } else {
                break;
            }
        }
        results.push((real_index, current_num.parse::<u32>().unwrap()));
        current_index = real_index + current_num.len();
    }
    results
}

fn sort_vec_by_index(vec: Vec<(usize, u32)>) -> Vec<(usize, u32)> {
    let mut new_vec = vec;
    new_vec.sort_by(|a, b| a.0.cmp(&b.0));
    new_vec
}

fn sorted_vec_to_str(vec: &Vec<(usize, u32)>) -> String {
    let mut result = String::new();
    for value in vec {
        result.push_str(value.1.to_string().as_str());
    }
    result
}
