use ndarray::{arr2, Array, Dim};

fn check_for_max(mut sums: Vec<i32>, input: Array<i32, Dim<[usize; 2]>>) -> i32 {
    for x in 0..4 {
        for y in 0..4 {
            let mut sum = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if j == 1 && (i == 0 || i == 2) {
                        sum += 0;
                    } else {
                        sum += input[[y + j, x + i]];
                    }
                }
            }
            sums.push(sum);
        }
    }

    let max_value = sums.iter().max().unwrap();
    *max_value
}
fn main() {
    let a = arr2(&[
        [1, 1, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0],
        [0, 0, 2, 4, 4, 0],
        [0, 0, 0, 2, 0, 0],
        [0, 0, 1, 2, 4, 0],
    ]);

    let b = arr2(&[
        [1, 1, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0],
        [0, 9, 2, -4, -4, 0],
        [0, 0, 0, -2, 0, 0],
        [0, 0, -1, -2, -4, 0],
    ]);

    println!(
        "max value of input 1: \n{:?}\n",
        check_for_max([].to_vec(), a)
    );
    println!(
        "max value of input 2: \n{:?}",
        check_for_max([].to_vec(), b)
    );
}
