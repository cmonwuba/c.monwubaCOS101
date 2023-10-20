fn main(){
   let p = 210_000.0;
   let r = 5.0;
   let n = 3.0;

   let x = 1.0 - (r/100.0);
   let y = f32:: powf(x,n);
   let a = p * y;
   println!("Value of TV after 3 years is {}", a);
}