use ::clap::{Parser, Subcommand, Args};
mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "b64ed - a simple CLI tool to encode/decode string to base64", long_about = "A CLI tool used to encode and decode base64 strings")]

struct CliSyntax {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode(Encode),
    Decode(Decode),
}

#[derive(Args)]
struct Decode {
    string: Option<String>
}

#[derive(Args)]
struct Encode {
    string: Option<String>
}

//usage: b64ed <string> <encode/decode>
fn main() {
    let cli_args = CliSyntax::parse();

    match &cli_args.command {
        Some(Commands::Encode(input)) => {
            match input.string {
                Some(ref _input) => {
                    let b64e = api::b64ed::encode(_input);
                    println!("{}", b64e);
                }
                None => {
                    println!("Please provide a string to encode");
                }
            }
        }
        Some(Commands::Decode(input)) => {
            match input.string {
                Some(ref _input) => {
                    let b64d = api::b64ed::decode(_input);
                    println!("{}",b64d);
                }
                None => {
                    println!("Please provide a string to decode");
                }
            }
        }
        None => {}
     }
}

