// let h_stack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
fn matcher(h_stack: &[i32]) -> String{

    for reference in h_stack.iter() {
        let item = reference;

        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    };

}

matcher([1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]);