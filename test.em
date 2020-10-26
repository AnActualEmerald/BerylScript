fn main(args) {
<<<<<<< HEAD
  temp = readln("Enter your name: ");
  //operations don't work in function calls for whatever reason
  res = "Hello there, " + temp;
  println(res);
  age = number(readln("How old are you? "));
  if(age > 50) {
    println("You're over the hill!");
  } elif (age <= 50) {
    println("So young :O");
  }
  read("Press enter to continue");
}

fn objects() {

}

class Person{
  fn ~init(self, name, age, job){
    self.name = name;
    self.age = age;
    self.job = job;
  }

  fn greet(self){
    res = "Hello, I'm " + self.name + " and I'm " + self.age;
  }
}
=======
  i = 0;
  while(i < 10){
    i++;
    println(i);
  }  
}
>>>>>>> dev
