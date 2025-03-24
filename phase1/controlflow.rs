fn main(){
let mut a = 5 ;
    let mut count :i32 = 0 ;
    println!("hello {}",a);
    a = 8 ;
    println!("hello {}",a);
    if   a > 7 {
        println!("correct");
    }else{
        println!("incorrect");
    }
     let condition : bool  =true;
     let number = if condition {5} else {2};     
     println!("{}",number);
    loop{
      count +=1;
      print!("{}",count);
      if count==7{
        break;
      }
    }
    println!("");
    let mut x = 0;
    while x<5 {
        print!("{}",x);
        x += 1;
    }
    println!("");
    for i in 0..5{
        print!("{}",i);
    }
    println!("");
    let y : i32 = 7;
    match y{
        1..=5 => println!("in 1 to 5"),
        6..=10 => println!("in 6-10"),
        _ => println!("x"),
    }

}