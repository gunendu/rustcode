enum Stringresult {
 StringOK(String),
 StringErr(String),
}

fn main(){
 let string = "gunendu";
  let Stringresult::StringOK(_,val) =  enummatch(string);
  println!("value is {}",val);
}

fn enummatch (greeting:&str) -> Stringresult {
 if greeting == "gunendu"
 {
   Stringresult::StringOK("Gunendu is okay".to_string())
 }
 else
 {
   Stringresult::StringErr("Gunendu is not okay".to_string())
 }
   
}
