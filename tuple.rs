fn main() {
  let (x,y) : (int,&str) = (5i,"gunendu");
  println!("x is {} {}",x,y);
  let (xincr,yincr) = returntwo(x);
  println!("xincr is {}",yincr);
}

fn returntwo(x:int) -> (int,int){
  (x+1,x+1)
}
