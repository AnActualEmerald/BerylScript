fn main(args) {
  println("Hey there");
  objects();
}

fn objects() {
  bob = new Person("Bob", 69, "Professional Weed Smoker");
  bob.greet();
  println(bob);
  i = bob.get_age();
  for(i > 0; i--){
    println("Happy birthday, " + bob.name);
  }
}

class Person{
  fn ~init(self, name, age, job){
    self.name = name;
    self.age = age;
    self.job = job;
  }

  fn ~display(self) {
    return "Hello, I'm " + self.name + " and I'm " + self.age;
  }

  fn greet(self){
    println("Hello, I'm " + self.name + " and I'm " + self.age);
  }

  fn get_age(self) {
    return self.age;
  }
}
