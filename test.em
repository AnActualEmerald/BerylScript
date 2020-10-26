fn main(args) {
  println("Hey there");
}

fn objects() {
  bob = new Person("Bob", 69, "Professional Weed Smoker");
}

class Person{
  fn ~init(self, name, age, job){
    self.name = name;
    self.age = age;
    self.job = job;
  }

  fn greet(self){
    println("Hello, I'm " + self.name + " and I'm " + self.age);
  }
}
