use std::fs;
use std::path::Path;

fn readfile(file_path: &Path) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Split text file into an interator and then map to a vector
    let rows = contents.lines();

    let mut v1: Vec<Vec<i32>> = vec![];

    for (i, row) in rows.enumerate() {
        // println!("row: {}", row);
        v1.push(Vec::new());
        for val in row.split_whitespace() {
            // println!("val: {}", val.parse::<i32>().unwrap());
            // println!("{:?}", v1[i]);
            v1[i].push(val.parse::<i32>().unwrap());
        }
    }

    return v1;
}

fn check_safe(report: &[i32]) -> i32 {
    // Get the diff between elements
    // Check all same sign
    // check abs all between 1 and 3
    //
    let diff: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();

    // println!("report: {:?}", report);
    // println!("diff:   {:?}", diff);

    if diff.iter().all(|&x| x >= 0) || diff.iter().all(|&x| x <= 0) {
        if diff.iter().all(|&x| matches!(&x.abs(), 1..4)) {
            // println!("Safe Steps\n");
            return 1;
        } else {
            // println!("Unsafe Steps\n");
            return 0;
        }
    }
    // println!("Unsafe mixed\n");
    return 0;
}

fn check_safe_2(report: &[i32]) -> i32 {
    if check_safe(report) == 1 {
        return 1;
    }

    // Use recursion to check removal and return when safe.
    for i in 0..report.len() {
        if check_safe(&[&report[..i], &report[i + 1..]].concat()) == 1 {
            return 1;
        }
    }
    return 0;
}

fn main() {
    let file_path = Path::new("example.txt");
    // let file_path = Path::new("input.txt");

    let reports = readfile(file_path);
    // println!("reports: {:?}", reports);

    // Part 1 and 2
    let mut nsafe1 = 0;
    let mut nsafe2 = 0;
    for report in reports {
        nsafe1 += check_safe(&report);
        nsafe2 += check_safe_2(&report);
    }
    println!("Part 1: {}", nsafe1);
    println!("Part 2: {}", nsafe2);
}
