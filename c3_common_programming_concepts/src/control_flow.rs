
pub fn control_flow() {
    // //if conditions inside let
    // let condition :bool = false;
    // let number = if condition { 5 } else { 6 };
    // println!("{number}");

    // // looped repetition
    // loop {
    //     println!("wowowowowo");
    // }

    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("result is {result}");

    // loop_labels();

    // while_loops();

    for_loops();
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number > -1 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFT OFFS");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for e in a {
        println!("the value is {}", e);
    }

    for number in (0..4).rev() {
        println!("{number}");
    }
    println!("LIF TOFF");
}