use std::fs;
use std::path::Path;

fn file2vecs (file_path: &Path) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Split text file into an interator and then map to a vector
    let rows = contents.lines();

    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];

    for row in rows {
        let mut x = row.split_whitespace();
        if let (Some(first), Some(second)) = (x.next(), x.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                v1.push(num1);
                v2.push(num2);
            }
        }
    }

    // Sort into ascending order
    v1.sort();
    v2.sort();

    return (v1, v2);
}


fn part1 (v1: &[i32], v2: &[i32]) -> i32 {
    // Iterate over and take absolute difference
    let mut dists: Vec<i32> = vec![];

    for i in 0..v1.len() {
        let dist = (v1[i] - v2[i]).abs();
        dists.push(dist);
    }
    // println!("dists: {:?}", dists);

    let part1: i32 = dists.iter().sum();
    return part1;
}


fn part2 (v1: &[i32], v2: &[i32]) -> i32 {
    let mut items: Vec<i32> = vec![];
    let mut count: Vec<i32> = vec![];
    for v2_val in v2 {
        if items.contains(&v2_val) {  // redundant & as v2 is a reference, but add for clarity as contains takes a reference.
            let index = items.iter().position(|&x| x == *v2_val).unwrap();
            count[index] += 1;
        } else {
            items.push(*v2_val);
            count.push(1);
        }
    }
    // println!("items: {:?}", items);
    // println!("count: {:?}", count);

    let mut part2: i32 = 0;
    for v1_val in v1 {
        if items.contains(&v1_val) {
            let index = items.iter().position(|&x| x == *v1_val).unwrap();
            part2 += *v1_val * count[index];
        }
    }

    return part2;
}


fn main() {
    // let file_path = Path::new("example.txt");
    let file_path = Path::new("input.txt");

    let (v1, v2) = file2vecs(file_path);

    let part1 = part1(&v1, &v2);
    println!("Part 1: {part1}");

    let part2 = part2(&v1, &v2);
    println!("Part 2: {part2}");

}
