struct Life{
    name : String ,
    age : i32,
    happy: bool ,
}

struct Check(i32,i32,i32);

struct Square{
    side:i32,
}

impl Square{
    fn area(&self) -> i32{
        self.side*self.side
    }
    fn perimeter(&self) -> i32{
        self.side*4
    }
}

enum Rpst{
    Rock,
    Paper,
    Scissor,
    Thread,
}

enum Message{
    Quit,
    Move,
    Write,
}

impl Message{
    fn choose(&self) -> &str{
        match self{
            Message::Quit => "quit",
            Message::Move => "move",
            Message::Write => "write",
        }
    }
}


fn main(){
    let h1 = Life{
        name : String::from("Atul"),
        age : 20,
        happy : false,
       };
    
    
       println!("{} {}",h1.name,h1.age);
       if h1.happy{
        println!("good");
       }else{
        println!("be happy");
       }
    
       let h2: Life = Life{
        name : String::from("Devansh"),
        ..h1
       };
       println!("{} {}",h2.name,h2.age);
    
    
      let test = Check(4,8,12);
       println!("{} {} {}",test.0,test.1,test.2);
      let sq = Square { side : 75};
      println!("{} {}",sq.area(),sq.perimeter());
      let chc1 = Rpst::Rock;
      let chc2 = Rpst::Paper;
      let chc3= Rpst::Scissor;
      let chc4= Rpst::Thread;
      match chc1{
        Rpst::Rock => println!("Rock"),
        Rpst::Paper => println!("Paper"),
        Rpst::Scissor => println!("Scissor"),
        Rpst::Thread => println!("Thread"),
      }
      match chc2{
        Rpst::Rock => println!("Rock"),
        Rpst::Paper => println!("Paper"),
        Rpst::Scissor => println!("Scissor"),
        Rpst::Thread => println!("Thread"),
      }
      match chc3{
        Rpst::Rock => println!("Rock"),
        Rpst::Paper => println!("Paper"),
        Rpst::Scissor => println!("Scissor"),
        Rpst::Thread => println!("Thread"),
      }
      match chc4{
        Rpst::Rock => println!("Rock"),
        Rpst::Paper => println!("Paper"),
        Rpst::Scissor => println!("Scissor"),
        Rpst::Thread => println!("Thread"),
      }
       let msg = Message::Write;
    //  match msg{
    //      Message::Write(text) => println!("{}",text),
    //    _ => println!("something"),
    //   }
    println!("{}",msg.choose());
}