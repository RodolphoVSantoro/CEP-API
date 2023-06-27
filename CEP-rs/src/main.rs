mod cep;
mod rb_file;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // let bytes = rb_file::create_rb_bytes("data/ceps.txt", true);

    // rb_file::write_bytes(bytes, "data/cep.bin", true);

    let ceps_bytes = rb_file::read_bytes("data/cep.bin", true);
    let ceps_bitmap = rb_file::create_rb_from_bytes(ceps_bytes, true);

    let app = Router::new()
        .route("/cep/:cep_str", get(cep::find_cep))
        .with_state(ceps_bitmap);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
