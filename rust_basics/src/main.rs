const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}!", OUR_COURSE);

    // Stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // for loop
    for i in 0..=y{
        if i != 4 {
            print!("{}, ", i);
        }   else
        {
            println!("{} ", i);
        }
    }

    // Mutation with integer
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but it is now {}", z);

    // Float
    let freezing_temp: f64 = -2.0;
    println!("freezing temp is {}", freezing_temp);

    // Boolean
    let is_zero_reminder: bool = 10 % 4 != 0;
    println!("is_zero_reminder is {}", is_zero_reminder);

    // Char
    let my_char: char = 'z';
    println!("my_char is {}", my_char);
    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    // Array of floats
    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n: f32| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

}
