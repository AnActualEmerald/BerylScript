fn main() {
  print "Hello world";
  a = 10;
  b = 10 * (((((((5+1)-3))))));
  c = a + b;
  print hello_world(a);
  a_and_one = add_one(a);
  print a_and_one;
  print b;
  print c;
  print hello_world("Returning a value");
}

fn add_one(val){
  return val + 1;
}

fn hello_world(saying) {
    print saying;
    return "Done";
}
