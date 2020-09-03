fn main() {
  i = 5;
  for(i < 10; i = i + 1){
    print "hey there";
    print i;
  }

  print "Loop's done";

  i = 0;
  while i < 10 {
    print "Hello, world! The year is 2020!";
    print "6, and then 9, is my favorite number";
    print 69;
    print exponent(6, 9);
    
    print i;
    i = i + 1;
  }

  print i;
  print "This will now print the number two";
  print returns_2();
}

fn exponent(base, power){
  i = 1;
  res = 0;
  while i <= power {
    res = res + base;
    i = i + 1;
  }
  return res;
}

fn returns_2() {
  return 1 + 1;
}