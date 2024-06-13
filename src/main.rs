use std::io;   
use rand::Rng;
use std::env;
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

fn sound(track : &str){
   // use std::fs::File;
   // use std::io::BufReader;
   // use rodio::{Decoder, OutputStream, source::Source};

   // // Get an output stream handle to the default physical sound device
   // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
   
   
   // let file = BufReader::new(File::open(track).unwrap());
   // let source = Decoder::new(file).unwrap();
   // let _ = stream_handle.play_raw(source.convert_samples());
   // std::thread::sleep(std::time::Duration::from_secs(5));

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

fn chat(){
   let rng: i8 = rand::thread_rng().gen_range(1..=12);
   match rng {
      1 => {mama(); sound("audio/audio.ogg");},
      2 => {println!("Ti takovou napalim že us se nenapiješ"); sound("audio/audio.ogg");},
      3 => {println!("Hmm."); sound("audio/audio.ogg");},
      4 => {println!("Ty seš fakt charakter."); sound("audio/audio.ogg");},
      5 => {println!("To je hafo dobrý"); sound("audio/audio.ogg");},
      6 => {println!("Ty mrrrdko"); sound("audio/audio.ogg");},
      7 => {println!("Ty zmrde"); sound("audio/audio.ogg");},
      8 => {println!("Ty jsi píči a už se nenapiješ."); sound("audio/audio.ogg");},
      9 => {println!("Ty jsi píči"); sound("audio/audio.ogg");},
      10 => {println!("Nabí mi"); sound("audio/audio.ogg");},
      11 => {sibir(); sound("audio/audio.ogg");},
      _ => {tricet5(); sound("audio/audio.ogg");},
   }

   let mut input: String = String::new();
   io::stdin().read_line(&mut input).unwrap();
   end(input);
   chat();
}

fn main() {
   env::set_var("RUST_BACKTRACE", "1");
   clearscreen::clear().expect("failed to clear screen");

        println!(r#"
        
███    ██  ██████   ██████       █████  ██ 
████   ██ ██       ██    ██     ██   ██ ██ 
██ ██  ██ ██   ███ ██    ██     ███████ ██ 
██  ██ ██ ██    ██ ██    ██     ██   ██ ██ 
██   ████  ██████   ██████      ██   ██ ██ 
                                           
                                           

                                      
                                      
            "#);
   chat(); 
}
