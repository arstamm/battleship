fn main() {
    let mut array : [[i32; 10]; 10] = [[0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            println!(array[i][j])
        }
    }
}

fun attack(array: &mut [[i32; 10]; 10], row: usize, col: usize) {
    if row < 10 && col < 10 {
        array[row][col] += 1;
    }
}