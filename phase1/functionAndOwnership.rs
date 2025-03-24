

fn main(){
    greet("1");
    let sum  = add(5,7);
    println!("Sum {}",sum);
    let s1  = String::from("hello");
    let s2 = s1;
    println!("{}",s2);
    // println!("{}",s1);
}


fn greet(name :&str){
    println!("greet func {}",name);
}

fn add(x :i32 , y :i32) -> i32 {
    x+y
}