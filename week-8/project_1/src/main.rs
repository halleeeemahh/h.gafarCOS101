use std::io;

fn vadmin(){
    let vadmin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    println!("What is your current position, Enter: \n 0 for Intern, \n 1 for Administrator, \n 2 for Senior Administrator, \n 3 for Office Manager, \n 4 for Director, \n 5 for CEO");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid Input");
    let role = vadmin[irole];
    println!("Your role is {}", role);
    let mut iexp =String::new() ;
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid Input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS Level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
         println!("Your APS Level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
         println!("Your APS Level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
         println!("Your EL1 Level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp<=13{
         println!("Your EL2 Level is EL2 10-13");
    }else if irole == 5 || iexp>= 13 {
         println!("SES");
    }

}

fn vacademic(){
    let vadmin = vec!["-","Research Assistant","phD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    println!("What is your current position, Enter: \n 0 for -, \n 1 for Research Assistant, \n 2 for phD Candidate, \n 3 for Post-Doc Researcher, \n 4 for Senior Lecturer, \n 5 for Dean");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid Input");
    let role = vadmin[irole];
    println!("Your role is {}", role);
    let mut iexp =String::new() ;
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid Input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS Level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
         println!("Your APS Level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
         println!("Your APS Level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
         println!("Your EL1 Level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp<=13{
         println!("Your EL2 Level is EL2 10-13");
    }else if irole == 5 || iexp>= 13 {
         println!("SES");
    }

}

fn vlawyer(){
    let vadmin = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    println!("What is your current position, Enter: \n 0 for Paralegal, \n 1 for Junior Associate, \n 2 for Associate, \n 3 for Senior Associate 1-2, \n 4 for Senior Associate 3-4, \n 5 for Partner");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid Input");
    let role = vadmin[irole];
    println!("Your role is {}", role);
    let mut iexp =String::new() ;
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid Input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS Level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
         println!("Your APS Level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
         println!("Your APS Level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
         println!("Your EL1 Level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp<=13{
         println!("Your EL2 Level is EL2 10-13");
    }else if irole == 5 || iexp>= 13 {
         println!("SES");
    }

}

fn vteacher(){
    let vadmin = vec!["Placement","Classroom teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
    println!("What is your current position, Enter: \n 0 for Placement, \n 1 for Classroom teacher, \n 2 for Senior Teacher, \n 3 for Leading Teacher, \n 4 for Deputy Principal, \n 5 for Principal");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid Input");
    let role = vadmin[irole];
    println!("Your role is {}", role);
    let mut iexp =String::new() ;
    println!("How many years of experience do you have ?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid Input");
    if irole == 0 || iexp>= 1 && iexp <=2 {
        println!("Your APS Level is APS 1-2");
    }else if irole == 1 || iexp>= 3 && iexp <=5{
         println!("Your APS Level is APS 3-5");
    }else if irole == 2 || iexp>= 5 && iexp <=8{
         println!("Your APS Level is APS 5-8");
    }else if irole == 3 || iexp>= 8 && iexp <=10{
         println!("Your EL1 Level is EL1 8-10");
    }else if irole == 4 || iexp>= 10 && iexp<=13{
         println!("Your EL2 Level is EL2 10-13");
    }else if irole == 5 || iexp>= 13 {
         println!("SES");
    }

}
fn main() {
    let mainv = vec!["Office Admin","Academic","Lawyer","Teacher"];
    println!("Select your sector, Enter: \n0 for Office Admin, \n1 for Academic, \n2 for Lawyer, \n3 for Teacher");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let vmain:usize = input1.trim().parse().expect("Invalid Input");
    let sector = mainv[vmain];
    match sector.trim(){
        "Office Admin" => {
            vadmin() }

        "Academic" => {
            vacademic()}

        "Lawyer" => {
            vlawyer()}

        "Teacher" => {
            vteacher()}
        _ => {
            println!("Invalid Input");
        }    
    }
}
