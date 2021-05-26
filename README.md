# EmeraldScript  ![Build](https://github.com/AnActualEmerald/EmeraldScript/workflows/Build/badge.svg) [![Tests](https://github.com/AnActualEmerald/EmeraldScript/actions/workflows/Test.yml/badge.svg)](https://github.com/AnActualEmerald/EmeraldScript/actions/workflows/Test.yml)

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

# Getting started

To get started using EmeraldScript, you'll want to do `cargo install --git https://github.com/AnActualEmerald/EmeraldScript.git` to install the binary that will actually run the code (I like to call it the gem because it's fun to name things). Next, I would highly suggest doing `cargo install --git https://github.com/AnActualEmerald/beryl.git` to get a more user friendly CLI and a repl. Beryl also comes with some example scripts that might be a good place to start. Once you get both of the binaries installed you should be good to go, assuming that your `~/.cargo/bin/` directory ~~or whatever it is on windows~~ is in your path variable. Just type `beryl` into your command line to open the repl, or `beryl help` to see the other options.

Because the gem itself is an executable binary, you can also use it to run scripts via a wizzbang line at the top of a text file (think `#!/bin/bash`). On linux at least, it should be as simple as adding `#!/path-to-the-binary` at the top of your script, and making that script an executable file with chmod. I'm sure anyone who happens across this page already knows about that, but I just think it's neat.

One last thing: in the event you want or need to put the gem somewhere else on your system, you can set the GEM_BIN environment variable to point directly at the gem and beryl will be figure it out. This way, if you need a specific gem version or just like to keep all of your binaries in one spot that isn't in PATH for some reason, you can do that. Also, `beryl run` has a `--gem-path` option too, so there's really no excuse. 




Find me on [twitter](https://twitter.com/KevahnGee) or join my [discord](https://discord.gg/bkQJeCH)
