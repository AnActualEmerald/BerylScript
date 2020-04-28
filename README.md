# EmeraldScript ![Build](https://github.com/AnActualEmerald/EmeraldScript/workflows/Build/badge.svg)

### What is EmeraldScript?

EmeraldScript is an interpreted programming language. The interpreter itself is written in rust, and has kind of been my intro into some more advanced ideas in rust. This project is mostly just a toy, and currently isn't really intended for public consumption.

### How is EmeraldScript?

It's doing okay. Currently it supports assigning and reading variables, as well as functions and while loops. More ways to write loops as well as actual control flow should be coming soon(tm).

### What does it look like?

Right now, EmeraldScript's syntax is a combination of rust, js, and python. This isn't exactly by design, and will probably change in the future.
In [alpha version 2](https://github.com/AnActualEmerald/EmeraldScript/releases/tag/v0.2-alpha) I included a zip with a few very basic example files. 
Here's a look at one:

```
fn main(){
  i = "Hello, World!";
  print i;
}
```
This is, obviously, a hello world program. There are a few things to note:
1. Variable declaration doesn't require a keyword
2. "print" is a keyword, and not a function
3. The code is in a main method

The first two points will likely change as I get more of the most basic features ironed out and begin to nail down more of the design aspects of the language. The third is something that I want to keep, as I feel like having a main function makes it clear where things will actually start executing (of course, if you look at the way I'm doing this you'll see that anything outside the main function will execute as well).

If you want to contribute ~~for some reason~~ or run in to some kind of bug or want to request a feature, feel free to get in touch. Though be warned that I'm not considering this project very high priority or very serious, so I may be slow to respond.

---

Find me on [twitter](https://twitter.com/KevahnGee) or join my [discord](https://discord.gg/bkQJeCH)
