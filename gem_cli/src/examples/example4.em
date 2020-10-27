fn main(args) {
    name = readln("What is your name: ");
    age = readln("How old are you? ");
    bob = new Person(name, age);

    println(bob);
    bob.greet();
}

class Person{
    fn ~init(self, name, age) {
        self.name = name;
        self.age = age;
    }

    fn ~display(self) {
        return "Name: " + self.name + " Age: " + self.age;
    }

    fn greet(self) {
        println("My name is " + self.name + " and I am " + self.age + " years old");
    }
}