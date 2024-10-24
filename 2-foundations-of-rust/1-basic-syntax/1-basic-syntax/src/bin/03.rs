fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut largest = input[0];
    let mut smallest = input[0];

    for &num in input.iter() 
    {
        if num > largest 
        { largest = num; }
            else 
            { smallest = num ;}
    }

    println!("{} is largest and {} is smallest", largest, smallest);
}
