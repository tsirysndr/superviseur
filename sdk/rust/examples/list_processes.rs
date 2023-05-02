use superviseur_client::client::connect;

fn main() {
    connect().project("deno-example").processes();
}
