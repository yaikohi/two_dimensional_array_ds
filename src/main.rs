use ndarray::{arr2, Array, Dim};

fn check_for_max(mut sums: Vec<i32>, input: Array<i32, Dim<[usize; 2]>>) -> i32 {
    for x in 0..4 {
        for y in 0..4 {
            let mut sum = 0;

            sum += input[[y, x]];
            sum += input[[y, x + 1]];
            sum += input[[y, x + 2]];

            sum += input[[y + 1, x + 1]];

            sum += input[[y + 2, x]];
            sum += input[[y + 2, x + 1]];
            sum += input[[y + 2, x + 2]];

            sums.push(sum);
        }
    }

    let max_value = sums.iter().max().unwrap();

    println!("max value: {:?}", max_value);

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

    check_for_max([].to_vec(), a);
    check_for_max([].to_vec(), b);
}
