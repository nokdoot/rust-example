fn main() {
    let strr = String::from("hello world");
    println!("{}", first_word(&strr));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}