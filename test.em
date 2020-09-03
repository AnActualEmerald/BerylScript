fn main() {
  i = 0;
  for(i < 10; i = i + 1){
    print "Hey there";
    print i;
  }

  print "Loop's done";
  control_flow();
}

fn control_flow(){
  if(5 == 5){
    print "This was true";
  }
  else {
    print "This was false";
  }

  if(5 != 5){
    print "This was true";
  }
  else {
    print "This was false";
  }

  if (5 != 5) {
    print "5 doesn't equal 5 I guess";
  } elif (5 != 6) {
    print "5 doesn't equal 6";
  } else {
    print "Nothing is real";
  }

}
