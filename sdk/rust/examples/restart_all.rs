use superviseur_client::client::connect;

fn main() {
    connect().project("deno-example").restart_all();
}
