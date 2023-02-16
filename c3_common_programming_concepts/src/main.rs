mod accessing_arrays;
mod functions;
mod control_flow;
mod fahrenheit_to_celsius;

fn main() {
    // let x = 5; // immutable
    //
    // let x = x + 1;  // shadowing
    //
    // { // inner scope
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //
    // println!("{}", THREE_HOURS_IN_SECONDS);

    // // immutable type changes on demand
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("spaces={spaces}");
    //
    // // floating point things
    // let x = 2.9; // default inherent f64
    //
    // let y: f32 = 3.2; //explicit declaration
    //
    // println!("x={x}//f64 y={y}//f32");
    //
    // // number literals
    // let dec = 98_222_200;
    // let byte = b't';
    // println!("{} {}", dec, byte);
    //
    // // boolean types
    // let t = true;
    // let f: bool = false;
    // println!("t={t} f={f}");
    //
    // // character types
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("c={c} z={z} heart_eyed_cat={heart_eyed_cat}");
    //
    // // compound types for grouping variables into one type
    // //tuples:
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (t1, t2, t3) = tup;
    // println!("{} {} {}", t1, t2, t3);
    //
    // // access by index
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
    // println!("{} {} {}", five_hundred, six_point_four, one);
    //
    // // array types
    // let array:[i32; 5] = [1, 2, 3, 4, 5]; // manually specify array length if you wish
    // let array2 = [3; 5]; // fill array with five "3"'s

    // activate sub chapters
    // accessing_arrays::arrarr();
    // functions::function_one();
    control_flow::control_flow();

    fahrenheit_to_celsius::f_c(3.0);

}
