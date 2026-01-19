use clap::Parser;
use tashi_vertex::KeySecret;

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

        println!("Secret: {secret}");
    } else if let Some(public) = args.public {
        // ...
    } else {
        println!("Usage: key-parse --secret KEY\n");
        println!("   or: key-parse --public KEY\n");
    }

    Ok(())
}
