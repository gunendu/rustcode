fn main ()
{
  let y:int = foo(3);
  println!("y is this {}",y);
}

fn foo(x:int) -> int {
 x + 1
}
