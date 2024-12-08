use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

fn readfile(file_path: &Path) -> String {
    let contents = fs::read_to_string(file_path).expect("Could not read the file");

    // Read all text into one long string, ignoring newlines
    // Note: I feel the instructions were unclear here and I spent a lot of
    // time getting wrong answers by treating each line as a separate part.
    let corrupted_prog = contents.replace("\n", "");

    return corrupted_prog;
}

fn result_from_section_1(section: &str) -> i32 {
    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();

    // Extract all mul(x, y) from the section and store as vec of str
    // Loop over extracted mul(x, y), extract x and y, and multiply
    let num: i32 = re_mul
        // Extract the mul(x, y) from the section
        .find_iter(section)
        .map(|x| x.as_str())
        // Extract the x and y from the mul(x, y) and take product
        .map(|x| {
            let y: i32 = re_num
                .find_iter(x)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .product();
            return y;
        })
        // Sum all the products
        .sum();

    return num;
}

fn result_from_section_2(section: &str) -> i32 {
    // Cannot use the following as it will eat everything between the first
    // don't and last do.
    // let re_dead = Regex::new(r"don't\(\).+do\(\)").unwrap();

    // Split on do() to get blocks to definitely execute, then split on don't()and operate only
    // non the first (.next()) piece.
    // on the first part.
    let splitline_and_sum: i32 = section
        .split_inclusive("do()")
        .map(|x| x.split_inclusive("don't()").next().unwrap())
        // Run part 1 on each of the 'useable' blocks from the section and sum
        .map(result_from_section_1)
        .sum();

    return splitline_and_sum;
}

fn main() {
    // provide file as command line arg
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let file_path = Path::new(&args[1]);

    let corrupted_prog = readfile(file_path);

    // Part 1
    let part1: i32 = result_from_section_1(&corrupted_prog);
    // Part 2
    let part2: i32 = result_from_section_2(&corrupted_prog);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
