pub fn run() {
  // TUPLES group values with a variety of types
  let mut person: (&str, &str, i8) = ("yetson", "warsaw", 49);
  person.1="city";
  println!("TUPLES---");
  println!("{} is from {} and is {}", person.0, person.1, person.2);
  
  // ARRAY groups values of a single type
  // has some metadata associated with it, like its length. A tuple does not.
  // possible looping & iterating (single type)
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  numbers[2] = 20;
  
  println!("ARRAYS---");
  println!("{:?}", numbers); // print whole by debug
  println!("Single value: {}", numbers[0]); // print one
  println!("Array Length: {}", numbers.len()); // Get array length
  println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); // Arrays are stack allocated
  
  // Array slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
  
  // VECTOR
  // ~array collection type that is allowed to chgange in SIZE
  // instead of [i32; 5] is Vec<i32> and vec![];
  let mut numbers: Vec<i32> = vec![6, 7, 8, 9];
  
  numbers[2] = 20;  // Re-assign value
  numbers.push(5);  // Add on to vector
  numbers.push(6);
  numbers.pop();  // Pop off last value
  
  println!("VECTORS---");
  println!("{:?}", numbers);
  println!("Single Value: {}", numbers[0]);  // Get single val
  println!("Vector Length: {}", numbers.len()); // Get vector length
  println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));   // Vectors are heap allocated

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Vector Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number iteration: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec mutated by for loop: {:?}", numbers);
}