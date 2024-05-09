use bitcoincore_rpc::{Auth, Client};
use bitcoincore_rpc::RpcApi;

fn init_rpc_client() -> Client {
    let rpc_user = "polaruser";
    let rpc_password = "polarpass";

    Client::new(
        "http://127.0.0.1:18443",
        Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
    )
    .expect("Failed to create RPC client")
}


fn main() {
    let client = init_rpc_client();
    println!("Client: {:?}", client.get_blockchain_info());
    println!("Client: {:?}", client.get_block_count());
    println!("Client: {:?}", client.get_block_hash(0));
}