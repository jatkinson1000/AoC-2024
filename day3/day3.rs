use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

fn readfile(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Could not read the file");

    // Split text file into an interator and then map to a vector
    let sections = contents.lines().map(String::from).collect();

    // println!("sections: {:?}", sections);

    return sections;
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
            //.collect();
            // println!("y: {:?}", y);
            return y;
        })
        // Sum all the products
        .sum();

    // println!("num: {:?}", num);
    return num;
}

fn main() {
    // provide 'real' as command line arg for real input, otherwise use example.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let file_path = Path::new(&args[1]);

    let sections = readfile(file_path);
    // println!("Sections: {:?}", sections);

    // Part 1
    let part1: i32 = sections.iter().map(|x| result_from_section_1(x)).sum();

    println!("Part 1: {}", part1);
}
