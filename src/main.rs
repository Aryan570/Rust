use std::io;
use std::collections::HashMap;
use std::cmp;
fn main() {
    let mut vect : Vec<u32> = Vec::new();
    let mut map = HashMap::new();
    println!("Enter the length of the Array");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("There was an error");
    let length :usize = length.trim().parse().expect("Invalid Input");
    let mut idx = 0;
    while idx < length {
        let mut ip = String::new();
        io::stdin().read_line(&mut ip).expect("There was an error");
        let num :u32 = ip.trim().parse().expect("Invalid Input");
        vect.push(num);
        idx += 1;
    }
    println!("Your array is {:?}",vect);
    vect.sort();
    println!("Array after sorting is {:?}",vect);
    let median = vect.get(length/2);
    match median {
        Some(median) => println!("The median of the array is {}",median),
        None => println!("No median of the array"),
    }
    for num in vect {
        let count = map.entry(num).or_insert(0);
        *count +=1;
    }
    println!("{:?}",map);
    let mut maxi = 0;
    for value in map.values(){
        maxi = cmp::max(maxi,*value);
    }
    println!("Max number of time any element occured is {}",maxi);
    for (key,value) in map.iter(){
        if *value == maxi {
            println!("Mode of the Array is {}", *key);
            break;
        }
    }
}
