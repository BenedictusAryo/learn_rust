fn main() {
    // For loop
    // for number in (1..5).rev() {
    //     println!("number: {number}")
    // }

    // String Slice
    // let my_string = String::from("hello world");
    // let my_str = &my_string[1..5];

    // Struct
    // Derive Debug for print
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let mut person_1 = Person{name:"Aryo".to_string(), age:25};
    
    println!("{}", person_1.name);
    println!("{}", person_1.age);

    // modify Age
    person_1.age = 20;
    println!("{}", person_1.age);

    impl Person {
        fn add_age(&mut self, add_age: u8) {
            self.age += add_age;
        }
    }

    person_1.add_age(10);
    println!("After: {:?}", person_1);
}   
