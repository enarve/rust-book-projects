fn main() {
    // Main function
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is {y}");
    
    // print_labeled_measurement(4, 'h');
    let a = five();
    println!("{a}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}.");
}

fn five() -> i32 {
    5
}