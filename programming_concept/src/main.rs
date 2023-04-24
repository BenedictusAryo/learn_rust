fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    let y = plus_one(y);
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}