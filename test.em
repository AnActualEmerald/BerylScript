fn main() {
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