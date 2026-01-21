use clap::Parser;
use tashi_vertex::{KeyPublic, KeySecret};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    secret: Option<String>,

    #[clap(long)]
    public: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(secret) = args.secret {
        // Decode and parse the secret key from DER encoded Base58 string
        let secret: KeySecret = secret.parse()?;
        let public = secret.public();

        println!("Secret: {secret}");
        println!("Public: {public}");
    } else if let Some(public) = args.public {
        let public: KeyPublic = public.parse()?;

        println!("Public: {public}");
    } else {
        println!("Usage: key-parse --secret KEY\n");
        println!("   or: key-parse --public KEY\n");
    }

    Ok(())
}
