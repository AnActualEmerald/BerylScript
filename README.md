# EmeraldScript  ![Build](https://github.com/AnActualEmerald/EmeraldScript/workflows/Build/badge.svg)

### What is EmeraldScript?

EmeraldScript is an interpreted programming language. The interpreter itself is written in rust, and has kind of been my intro into some more advanced ideas in rust. This project is mostly just a toy, and currently isn't really intended for public consumption.

### How is EmeraldScript?

It's doing great! Currently all of the basic features you would expect are supported and stable, including for and while loops, if-elif-else statements, functions, variables, and even objects in the style of Python. You can check out the [issues](https://github.com/AnActualEmerald/EmeraldScript/issues) to see what I'm planning to work on next, or to request something.

### What does it look like?

Right now, EmeraldScript's syntax is a combination of Rust, JS, and Python. If you run the `examples` command in the CLI, it will generate a few example files to help get you started (You can also run the CLI itself to open a REPL to play around in).
Here's a look at one:

```
fn main(args){
  i = "Hello, World!";
  println(i);
}
```
This is, obviously, a hello world program. It showcases the use of a main function as an entry point that also takes in arguments from the command line, as well as the relatively new `println` function, which used to be a keyword. At this point I don't forsee any major syntax changes, and I may abandon the idea of enforcing typing altogether at well. 

If you want to contribute ~~for some reason~~ , run in to some kind of bug, or want to request a feature, feel free to get in touch. Though be warned that I'm not considering this project very high priority or very serious, so I may be slow to respond.

---

Find me on [twitter](https://twitter.com/KevahnGee) or join my [discord](https://discord.gg/bkQJeCH)
