use std::io;   
use rand::Rng;
//use colored::Colorize;


//Ends the program when a Siberia is given
fn end(end:String){
   if end.trim() == "Siberia"{
      std::process::exit(0);
   }
   else{
      return;
   }
}


//Zepta se jestli mama slape
fn mama(){
   println!("Tak co chlape, co máma?");
   let _ =io::stdin().read_line(&mut String::new()).ok(); //ask for input, throwaway
   println!("Furt šlape?");
}


fn tricet5(){
   let mut string: String = String::new();
   println!("Řekni 35");
   io::stdin().read_line(&mut string).ok();
   end(string.clone());
   println!("Jdi do píči {string}");
}

fn sibir(){
   println!("Hej dáš si sibiř?");
   let mut ask_sibir: String = String::new();
   io::stdin().read_line(&mut ask_sibir).unwrap();
   if ask_sibir.trim() == "jo"{
      println!("Tak to seš borec")
   }
   else{
      println!("Tak to jseš slabá mrdka")
   }

}

fn main() {
        clearscreen::clear().expect("failed to clear screen");

        println!(r#"
        ███████ ██ ██      ██ ██████  
        ██      ██ ██      ██ ██   ██ 
        █████   ██ ██      ██ ██████  
        ██      ██ ██      ██ ██      
        ██      ██ ███████ ██ ██      
                                      
                                      
            "#);
                       
   let rng: i8 = rand::thread_rng().gen_range(1..=12);
   match rng {
      1 => mama(),
      2 => println!("Ti takovou napalim že us se nenapiješ"),
      3 => println!("Hmm."),
      4 => println!("Ty seš fakt charakter."),
      5 => println!("To je hafo dobrý"),
      6 => println!("Ty mrrrdko"),
      7 => println!("Ty zmrde"),
      8 => println!("Ty jsi píči a už se nenapiješ."),
      9 => println!("Ty jsi píči"),
      10 => println!("Nabí mi"),
      11 => sibir(),
      _ => tricet5(),
   }

   let mut input: String = String::new();
   io::stdin().read_line(&mut input).unwrap();
   end(input);
   main()

   
    
}
