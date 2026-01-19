use tashi_vertex::KeySecret;

fn main() {
    // generate a new secret key to use for this node when signing transactions
    let secret = KeySecret::generate();

    println!("Secret: {secret}");
}
