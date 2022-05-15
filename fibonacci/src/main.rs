fn main() {
    let mut numbers: Vec<u128> = vec![0, 1];

    // 186 is as high as you can go with u128 before overflows
    for count in 1..186 {
        numbers.push(numbers[count] + numbers[count - 1]);
    }
    println!("{:#?}", numbers)
}
