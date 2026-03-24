use codecrafters_shell::{run, utils::exit};

fn main() {
   match run() {
       Ok(()) => exit(0),
       Err(error) => {
            eprint!("{}", error);
            exit(1);
        }
    }
}
