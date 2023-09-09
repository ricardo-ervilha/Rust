fn main() {
    test_one();
    let sum = add_numbers(2, 3);
    println!("The sum is: {}", sum);

    //Um jeito diferente de atribuir valor para uma variável
    let number = {
        let x = 3;
        x + 1
    };
    println!("The value of number is: {}", number);
}

fn test_one(){
    println!("Test has been called...");
}

fn add_numbers(x : i32, y : i32) -> i32 {
    return x + y;
}
