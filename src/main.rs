use std::io;    


fn main() {
   let mut string: String = String::new();
   println!("Řekni 35");
   io::stdin().read_line(&mut string).expect("kys nigga");
   if string.trim()=="Siberia" {
      std::process::exit(0)
   }
   else{
      println!("Jdi do píči {string}");
   }
   // io::stdin().read_line(&mut String::new()).unwrap();

    main()
}