use std::io;
fn main() {
    println!("Welcome to Ernst & Young (EY) Global Limited");
    println!("How many applicant's do you want to interview");
    let mut numb_appl = String::new();
    io::stdin().read_line(&mut numb_appl).expect("Failed to read input");
    let numb_appl:u8 = numb_appl.trim().parse().expect("Invalid Input");
    let mut v:Vec<(String,u8)> = Vec::new();
    for i in 0..numb_appl {
        println!("Applicant {}",i + 1 );
    
    println!("Input your name");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1 = input1.trim().to_string();
    println!("Input the years of experience");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let input2:u8 = input2.trim().parse().expect("Invalid input");
    v.push((input1,input2))
}
if let Some((input1,input2)) = v.iter().max_by_key(|(_,input2)|input2){
    println!("The most experience worker is {} with {} years of work",input1, input2);
}
}
