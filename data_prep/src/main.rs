mod prove1; 
mod prove2; 
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
    GenProve1Inputs(P1Args), 
    GenProve2Inputs(P2Args),
}

#[derive(Args)]
struct P1Args{
    /// input file path 
    input_path: String,
    /// output file path
    output_path: String,
}

#[derive(Args)]
struct P2Args{
    /// input file path 
    input_path: String,
    /// output file path
    output_path: String,
    /// sk file path
    sk_path: String,
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
        Commands::GenProve2Inputs(args) => {
            let src_path = &args.input_path;
            let circom_json_path = &args.output_path;
            let sk_path = &args.sk_path;
            // restore the sk from sk_path
            let enc512 = prove2::EncInput512::new(src_path, sk_path);
            // gen circom input json
            enc512.gen_circom_json(circom_json_path);
        }
        _ => {
            println!("not implemented yet");
        }
    }
}
