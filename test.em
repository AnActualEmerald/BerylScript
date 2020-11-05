fn main(args) {	
	input = readln("Enter numbers to add up, or 'n' to stop: ");
	result = 0;
	while input != "n" {
    println("Got number: " + input);
	  if (number(input) == null) {
			println("That wasn't a number");
		}else {
			result = result + number(input);	
		}
		input = readln("> ");
	}

	println("All those added up are " + result);
}