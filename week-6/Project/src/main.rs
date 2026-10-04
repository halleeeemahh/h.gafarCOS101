use std::io;

fn main() {
    println!("MENU");
    println!();
    println!("P Poundo Yam/ Edinkaiko Soup - #3_200");
    println!();
    println!("F Fried Rice & Chicken - #3_000");
    println!();
    println!("A Amala & Ewedu Soup - #2_500");
    println!();
    println!("E Eba & Egusi Soup - #2_000");
    println!();
    println!("W White Rice & Stew - #2_500");
    
    println!("Enter food letter (P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");

    let choice = choice.trim().to_uppercase();

    let (name, price): (&str, u32) = match choice.as_str(){
        "P"=> ("Poundo Yam / Edinkaiko Soup", 3200),
        "F"=> ("Fried Rice & Chicken", 3000),
        "A"=> ("Amala & Ewedu Soup", 2500),
        "E"=> ("Eba & Egusi Soup", 2000),
        "W"=> ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid choice: please pick P, F,A, E, W");
            return;

        }
    };
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin()
    .read_line(&mut qty_input)
    .expect("Failed to read input");

    let quantity: u32 = match qty_input.trim().parse(){
    Ok(n) if n > 0 => n,
_ => {
            println!("Invalid choice: enter a whole number");
            return;
        }
};
     let total = price * quantity;
     println!("\nOrder: {} x {}", quantity, name);
     println!("Total: #{}", total);

     if total > 10000 {
        let discount = total as f64 * 0.05;
        let final_total = total as f64 - discount;
        println!("Discount (5%): N{:.2}", discount);
        println!("Amount to pay: N{:.2}", final_total);
    } else {
        println!("Amount to pay: N{}", total);
    }
}
