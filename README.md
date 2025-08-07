im sorry i coded the entire project in one commit im not familiar to using git

# CATASSEMBLY

the one. YOU CAN WRITE CODE INSTEAD OF DRAGGING BLOCKS!!!!!
its for a game in roblox. "CatWeb".

https://www.roblox.com/games/16855862021/CatWeb-Make-a-Website

and the discord server

https://discord.gg/XS9qtShkRz

check it out pls its very very cool
and check the wbesite for catassembly (inside of catweb) its `asm.rbx`
tysm

# USAGE

you should compile the codebase first. you can use the command `cargo run` or `cargo build`.
the command `cargo run` is better if you're trying to contribute and testing if your code works.
the command `cargo build` is better if you're going to use the code normally.

after it compiles, a file called "target" should occur. you can rename it, use it, whatever.
to use it :
```
./target example.cwa
```
note : you might need to give the file exec permissions.

you can import the transpiled code into your website

# SYNTAX

## declare events

examples

```
event WhenButtonPressed(object "globalid"): {
    log("oh oh z4");
}
```

```
event WhenWebsiteLoaded():
    log("no braces!!");
```

currently implemented :
`WhenWebsiteLoaded()`
`WhenButtonPressed(object)`

## actions

examples

```
...
{
    log("logged message");
    warn("warning");
    err("faulty code fix it now");
}
```

```
...
{
    set(var, 2):
    repeat 3 {
        add(var, 3);
        log("var is currently {var}");
    }
}
```

currently implemented :

`wait(seconds)`
`log(message)`
`warn(warning)`
`err(error)`
`set(variable, any)`
`add(variable, number)`
`sub(variable, any)`
`loop: ...`
`repeat number: ...`
`break`

# TODOS

- improve syntax
- add more blocks and events
- add more blocks and events
- add compile-time macros
- add compile-time optimizations
- add simple error checking
- ~~add the ability to read files~~
- add comment support

its also kinda outdated since i started creating this before the mega update and never touched it since??? idk

i would really appreciate it if you contribute (copy paste is boring and almost the entire job is copy pasting now)
