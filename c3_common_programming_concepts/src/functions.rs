pub fn function_one() { //declared public
    // another_function(5);
    // print_labeled_measurement(5, 'h');
    // expressive_statements();

    println!("{}", plus_one(five()));
}

fn another_function(x: i32) { // private but called from public function while being private
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// expressions vs statements
fn expressive_statements() {
    let y = {
        let a = "a"; // only in this scope
        println!("{a}");
        let x = 3;
        x + 3
    };

    println!("The value of y is {y}");
}

// functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}