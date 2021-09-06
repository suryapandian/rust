### Tour

https://tourofrust.com/

https://www.linkedin.com/learning/first-look-rust


```rs
  let a = 1;
  let b = 2;

  a_function(a,b) 
  /*
  even though the default type of a and b is i32
  in this case since we haven't explicitly declared it as i32
  rust infers the type from the function's parameters type.

  Later on if we try to use `a` as i32, we will get compiler error
  */

  fn a_function(a: u32, b:u32) {
  	.....
  }

```