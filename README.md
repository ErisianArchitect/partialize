# partialize

`partialize` is a simple partial application library written in Rust.

# Example
```rust
#[derive(Clone)]
struct Foo(&'static str);
fn foo(arg0: &str, arg1: Foo) {
    println!("{}, {}", arg0, arg1.0);
}
let partial1 = foo.partial1("test");
partial1(Foo("foo"));
partial1(Foo("bar"));
let partial2 = partial1.partial1_once(Foo("test"));
partial2();
let partial3 = partial1.partial1_clone(Foo("Hello"));
partial3();
partial3();
foo.partial1("test")(Foo("test"));
```

# Disclaimer
There's a lot of LSP overhead when there are a lot of generics involved, so this library only implements partial application for functions with up to 8 arguments. I've included the script for generating the code with the repository for your convenience.

**NO AI WAS USED IN THE CREATION OF THIS LIBRARY.**