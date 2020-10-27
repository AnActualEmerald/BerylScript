fn main(args) {
  println("Hey there");
  objects();
}

fn objects() {
  bob = new Person("Bob", 69, "Professional Weed Smoker");
  bob.greet();
}

class Person{
  fn ~init(self, name, age, job){
    println("Object initialized");
    self.name = name;
    //println(self);
    self.age = age;
    self.job = job;
    println("My name is " + self.name);
    println(self);
  }

  fn ~display(self) {
    return "mmmmm, funeee joke";
  }

  fn greet(self){
    println("Hello, I'm " + self.name + " and I'm " + self.age);
  }
}
