use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Hello;

fn main() {
    Hello::hello_macro();
}
