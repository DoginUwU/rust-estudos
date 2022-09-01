fn main() {
 copy();
 mutable();
 multiple_pointers(); 
}

// Copy string to another memory address
fn copy() {
  let s1 = String::from("Hello");
  let s2 = s1.clone();
  
  println!("{}, world | {}, luiz", s1, s2);
}

// Append string on a mutable string using pointer 
fn mutable() {
  let mut s1 = String::from("Hello");
  
  change_string(&mut s1);
    
  println!("{}", s1);
}
fn change_string(string: &mut String) {
    string.push_str(", everybody!");
}

fn multiple_pointers() {
  let mut s = String::from("Hello");  
  
  let r1 = &s;
  let r2 = &s;
  
  println!("{}, {}", r1, r2); // No problems :D
    
  let r3 = &mut s; // No problems, because r1 & r2 will not used after println
  r3.push_str(", dogs");
  println!("{}", r3);
}