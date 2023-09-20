//Versão 1 do programa

// fn main() {
//     let length1 = 50;
//     let width1 = 30;

//     println!("A área do retângulo é {} metros quadrados.", area(length1, width1));
// }

// fn area(length : u32, width : u32) -> u32{
//     length * width
// }

//Versão 2 do programa usando tuplas (Agrupa os dados)

// fn main() {
//     let rect1 = (50, 30);
//     println!("A área do retângulo é {} metros quadrados.", area(rect1));
// }

// fn area(rect : (u32, u32)) -> u32{
//     rect.0 * rect.1
// }

//Versão 3 do programa usando structs (Dá mais significado)

// #[derive(Debug)]
// struct Rectangle{
//     length: u32,
//     width: u32
// }

// fn main() {
//     let rect1 = Rectangle { length: 30, width: 50};
//     println!("rect1 é {:#?}", rect1);
//     println!("A área do retângulo é {} metros quadrados.", area(&rect1));
// }

// fn area(rect : &Rectangle) -> u32{
//     rect.length * rect.width
// }


//Versão 4 do programa utilizando métodos de structs

#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32
}

// fn main() {
//     let rect1 = Rectangle { length: 30, width: 50};
//     println!("rect1 é {:#?}", rect1);
//     println!("A área do retângulo é {} metros quadrados.", rect1.area());
// }

fn main(){
    let rect1 = Rectangle { length: 50, width: 30};
    let rect2 = Rectangle { length: 40, width: 10};
    let rect3 = Rectangle { length: 45, width: 60};

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}

impl Rectangle{
    fn area(&self) -> u32{
        self.length * self.width
    }
}
