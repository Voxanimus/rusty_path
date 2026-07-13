use std::collections::HashMap;
use random_word::Lang;

// Constants for exercise 1
const ARRAY_SIZE_1: usize = 32;
const ARRAY_SIZE_2: usize = 31;

// Constant for execise 2
const VOWELS: [&str; 6] = ["a", "e", "i", "o", "u", "y"];

fn exercise_1(list: [u8; ARRAY_SIZE_2]){
    
    // gather median value
    let mut v: Vec<u8> = Vec::new();
    
    for number in list{
        v.push(number);
    }
    
    v.sort();
    println!("{:?}", v);
    
    let median_pos = v.capacity()/2;
    let median_number = &v[median_pos];
    println!("{median_number}");
    
    // compute mode value
    let mut map:HashMap<u8, u8> = HashMap::new();
    
    for number in list{
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    
    let mut mode: (u8, u8) = (0, 0);
    for (number, occurence) in &map{
        if *occurence > mode.1{
            mode.1 = *occurence;
            mode.0 = *number;
        }
    }
    
    println!("{:?}", mode);
    
}

fn exercise_2(s: &str){
    
    let chars: Vec<char> = s.chars().collect();
    let mut pig_latin = String::new();
    
    if VOWELS.contains(&(&chars[0]).to_string().as_str()){
        pig_latin = String::from(s) + "-h"; 
    }else {
        pig_latin = String::from(&s[1..]) + "-" + &chars[0].to_string();
    }
    
    pig_latin += "ay";
    
    println!("{pig_latin}");
}

fn exercise_3(){
    // grosse flemme
}

fn main() {
    
    // Initialize exercise 1 array
    let random_array: [u8; ARRAY_SIZE_2] = rand::random();
    
    exercise_1(random_array);
    
    // Generate random word for exercise 2
    let word = random_word::get(Lang::En);
    
    exercise_2(word);
}
