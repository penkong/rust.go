// #![allow(dead_code)]
use std::mem;

fn main() {
  // println!("Hello, world!");
  // it is immutalbe 
  let a:u8 = 123; // or i8
  println!("a = {}" , a);

  let mut b:i8 = 0;

  println!("b = {}" , b);

  b = 43;

  println!("b = {}" , b);

  let mut c = 4354353;
  println!("c = {}, size = {} bytes", c , mem::size_of_val(&c));

  c = -1;

  println!("c = {}", c);

  let z:isize = 123; // isize or usize 
  // correspond to size of pointer
  let size_of_zed = mem::size_of_val(&z);
  println!("z = {}, takesup {} bytes, , {} bit os", z , size_of_zed, size_of_zed * 8);
  // it defines size of bit of os like 64bit or 32bit

  let d:char = 'x';
  println!("d = {}, size = {}", d , mem::size_of_val(&d));


  let e:f64 = 2.5;  // default behavior double precision
  println!("e = {}, size = {}", e , mem::size_of_val(&e));

  let f = false;
  println!("f = {}", f)
}
