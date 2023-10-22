// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    #[macro_export]  //需要将宏导出到其父模块中
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub use my_macro;  //需要将宏导出到其父模块中  
}

fn main() {
    use macros::my_macro;
    my_macro!();
}
