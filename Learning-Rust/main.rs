//helllo world
fn main(){
   let x;
   x = 5;
   println!("the value of x is {}", x);
   
   //constants = they are declareed using the const keyword

   //i(32) / u = unsigned this use binary data format (eg: 32, 64, 8, 16)
   //f(64) for floating point numbers = 6.4 decimals
   
const MAX_POINTS:i32 = 100_000;
println!("A constant, MAX_POINTS, has the value: {}", MAX_POINTS);

//shadowing

  let z = 5;
  println!("the value of z is {}", z);

  let z = z + 1; //6
    println!("z is shadowed, the new value is: {}", z);


     {
        // This shadow only exists inside this scope.
        let z = z * 2; //12
        println!("In the inner scope, z is: {}", z);
     }
    
    let spaces: &str = " ";
    println!("{} this is a space string", spaces);
    

    //tuples
     let a: [i32; 5] = [1, 2, 3, 4, 5];
    
     let first = a[0];
     let second = a[1];
     println!("Array 'a' elements: first = {}, second = {}", first, second);
     
     ///destructing our tuple
       let tup: (&str, f64, bool) = ("Hello, tuples!", 6.4, true);
       
       let (x, y, z) = tup;
       println!("Desc x = {}, y = {}, z = {}", x, y, z);
       
       //purpose of ownership is to help manage heap data
       
      let s1 = String::from("hello");
      let s2 = s1.clone();
      println!("s2 = {}", s2);
      
      
      let s3 = String::from("Rust");
      println!("s3 = {}", s3);
      
      
   another_function();
   print_value(17);
   add_five(10);
}

fn another_function(){
    println!("hello from another funtion")
}

fn print_value(x: i32){
    println!("the value passed is {}", x)
}

fn add_five(num: i32) -> i32{
     num + 5
}

