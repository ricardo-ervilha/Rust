fn main() {
    //Fatias permitem referenciar uma sequência contígua de elementos dentro de uma coleção ao invés de refenciar a coleção inteira. Fatias não se apropriam dos dados subjacentes.

    let mut s : String = String::from("hello world");

    let s2: &str = "hello world";

    let word: usize = first_word(s2);
   
}


fn first_word(s : &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i: usize, &item : u8) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..];
}