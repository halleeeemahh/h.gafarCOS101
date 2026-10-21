use std::io::Write;

fn main() {
   let header = "\nLager, Stout, Non-Alcoholic";
   let row1 = "\n33 Export, Legend, Maltina";
   let row2 = "\nDesperados, Turbo King, Amstel Malta";
   let row3 = "\nGoldberg, Williams, Malta Gold";
   let row4 = "\nGuider, , Fayrouz";
   let row5 = "\nHeineken, , ";
   let row6 = "\nStar, , ";

   let mut file = std::fs::File::create("Drinks Table.csv").expect("Create Failed");
   file.write_all(header.as_bytes()).expect("Write failed");
   file.write_all(row1.as_bytes()).expect("Write failed");
   file.write_all(row2.as_bytes()).expect("Write failed");
   file.write_all(row3.as_bytes()).expect("Write failed");
   file.write_all(row4.as_bytes()).expect("Write failed");
   file.write_all(row5.as_bytes()).expect("Write failed");
   file.write_all(row6.as_bytes()).expect("Write failed");
   println!("\nDrink Table has been created ");

}
