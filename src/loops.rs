// Loops - Used to iterate until a condition is met

pub fn run() {
  let mut count = 0;
  
  // Inf loop
  loop {
    count += 1;
    println!("number: {count}");
    // necessary condition to break loop
    if count == 20 {
      break;
    }
  }
  
  // while loop
  let mut count = 0;
 
  while count <=100 {
      if count % 15 == 0 {
        println!("fizzbuzz");
      } else if count % 3 == 0 {
        println!("fizz");
      } else if count % 5 == 0 {
        println!("buzz");
      } else {
        println!("{count}");
      }
      count += 1;
  }

  // for range
  for x in 0..100 {
    if x % 15 == 0 {
      println!("For fizzbuzz");
    } else if x % 3 == 0 {
      println!("For fizz");
    } else if x % 5 == 0 {
      println!("For buzz")
    } else {
      println!("{}", x);
    }
  }
}
