fn main(args) {
  i = [[10, ["a", "b", "c"], 30], 2, 3];
  println i[0][1][2];
  i[0][0] = true;
  println i;
  println args;
}