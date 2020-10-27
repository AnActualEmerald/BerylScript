fn main(args) {
    println(add(2, 3));
}

//Functions can be defined wih arguments that will allow data to be passed from
//one scope to another. As of v0.3-alpha, functions outside of the main function
//won't have access to any globally defined variables, or variables defined in an outer scope
fn add(left, right) {
    return left + right;
}