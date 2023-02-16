use std::io;

pub fn arrarr() {
    let a = [1, 2, 3, 4, 5];

    // expect panic if outside of bounds
    println!("pls enter an arry index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];

    println!("the value of the element at inded {index} is {element}");
}

