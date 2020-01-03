use std::collections::HashMap;

fn main() {

    let mut hm = HashMap::new();

    hm.insert(3 , "Hello");
    hm.insert(5, "World");

    let r =  hm.get(&4).unwrap_or(&"NoString");
    println!("{}", r);
    

    match "5".parse::<f32>() {
        Ok(v)=>println!("OK - {}", v),
        Err(e)=>println!("Error - {}", e),
    }
}