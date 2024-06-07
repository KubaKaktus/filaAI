use std::io;   
use rand::Rng;

// enum Hlasky{
//    Mama,
//    Tricet5
// }

fn end(end:String){
   if end.trim() == "Siberia"{
      std::process::exit(0);
   }
   else{
      return;
   }
}

fn mama(){
   println!("Tak co chlape, co máma?");
   let _ =io::stdin().read_line(&mut String::new()).expect("kys nigga");
   println!("Furt šlape?");
}

fn tricet5(){
   let mut string: String = String::new();
   println!("Řekni 35");
   io::stdin().read_line(&mut string).expect("kys nigga");
   end(string.clone());
   println!("Jdi do píči {string}");
}


fn main() {
   let rng: i8 = rand::thread_rng().gen_range(1..=3);
   match rng {
      1 => mama(),
      2 => println!("Ti takovou napalim že us se nopoasdíš"),
      _ => tricet5(),
   }

   let mut input: String = String::new();
   io::stdin().read_line(&mut input).unwrap();

   if input.trim() == "Siberia"{
      std::process::exit(0);
   }
   else {
       main();
   }
    
}