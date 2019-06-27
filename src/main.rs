// DotEnv
#[macro_use]
extern crate dotenv_codegen;

fn main() {
    println!("{}", dotenv!("AWS_ATHENA_DB").to_string());
    println!(dotenv!("AWS_ATHENA_DB").to_string());
}
