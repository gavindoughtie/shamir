use std::io::{stdin};

fn main(){
  let mut input = String::new();
  println!("Number of items to share?: ");
  stdin().read_line(&mut input).expect("error: unable to read user input");
  let item_count = input.trim().parse::<u32>().unwrap();

  input.clear();
  println!("Number of shares?: ");
  stdin().read_line(&mut input).expect("error: unable to read user input");
  let share_count = input.trim().parse::<u32>().unwrap();
  shamir::shamir_share(item_count, share_count);
}
