fn main() {
    loop {
        println!("Again");
    break;
    }
    let mut index = 0;
    let a = [1, 2, 3, 4, 5];
    while index < 5 {
        println!("There you go {}",a[index]);
        index += 1;
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

