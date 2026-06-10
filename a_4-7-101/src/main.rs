fn main(){
    let a = String::from("hello word");
    first_word(&a);
    
    println!("{a}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item)in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
       
    }
     s.len();
}