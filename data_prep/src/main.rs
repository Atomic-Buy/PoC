mod prove1; 

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenProve1Inputs(InputsArgs), 
    GenProve2Inputs(InputsArgs),
}

#[derive(Args)]
struct InputsArgs{
    /// input file path 
    input_path: String,
    /// output file path
    output_path: String,
}
fn main(){
    let cli = Cli::parse();

    // match cmd 
    match &cli.command{
        Commands::GenProve1Inputs(args) => {
            let src_path = &args.input_path;
            let circom_json_path = &args.output_path;
            // new a data
            let data = prove1::Data15K::new(src_path).unwrap();
            // gen circom input json 
            data.export_circom_json(circom_json_path).unwrap();
        }
        _ => {
            println!("not implemented yet");
        }
    }
}
