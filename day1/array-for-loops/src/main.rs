// TODO: remove this when you're done with your implementation
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = Default::default();
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }
    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!(" {}", matrix[i][j]);
        }
        println!();
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix original: ");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed: ");
    pretty_print(&transposed);
}

    
