use std::fmt::Display;

pub fn echo(arguments: &[impl Display]) {
    let mut inputs = arguments.iter();
    if let Some(input) = inputs.next() {
        print!("{input}");
    }

    for input in inputs {
        print!(" {input}")
    }

    println!("")
}
