fn main() {
    let cond = 2 < 3;

    let cond2 = true && cond;

    println!("{}", cond);
    println!("{}", cond2);

    let food = "fruit";

    if food == "cookie"{
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("That's sounds healthly!");
    } else{
        println!("Oh, that's too bad!");
    }
    
}
