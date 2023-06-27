mod queries;

use std::net::SocketAddr;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("error".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // build our application with a single route
    let app = Router::new().route("/json", get(get_cep)).with_state(pool);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3021));
    info!("Listening on {}", socket_addr);
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_cep(
    Query(query): Query<QueryValues>,
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    if query.cep.parse::<u32>().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            "CEP deve conter apenas números".to_string(),
        );
    }
    let mut transaction = pool.begin().await.unwrap();
    // let cep_str = BufReader::new(std::fs::File::open("data/ceps-minified-sorted.json").unwrap());
    // let serialized: Vec<Cep> = serde_json::from_reader(cep_str).unwrap();

    // let mut counter = 0;

    // for cep in serialized {
    //     let query = sqlx::query!(
    //         "INSERT INTO ceps (cep, faixas_cep, bairro_num, localidade_no_sem, localidade_num, localidade, localidade_subordinada, logradouro_dnec, logradouro_texto, nome_unidade, numero_localidade, situacao, tipo_cep, uf) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10 , $11, $12, $13, $14) RETURNING id",
    //         cep.cep,
    //         &cep.faixas_cep,
    //         cep.bairro_num,
    //         cep.localidade_no_sem,
    //         cep.localidade_num,
    //         cep.localidade,
    //         cep.localidade_subordinada,
    //         cep.logradouro_dnec,
    //         cep.logradouro_texto,
    //         cep.nome_unidade,
    //         cep.numero_localidade,
    //         cep.situacao,
    //         cep.tipo_cep,
    //         cep.uf
    //     ).fetch_one(&mut transaction).await.unwrap();

    //     let cep_id = query.id;

    //     for caixa_postal in cep.faixas_caixa_postal {
    //         let query = sqlx::query!(
    //             "INSERT INTO faixas_caixa_postal (cep_id, caixa_inicial, caixa_final) VALUES ($1, $2, $3)",
    //             cep_id,
    //             caixa_postal.caixa_inicial,
    //             caixa_postal.caixa_final
    //         ).execute(&mut transaction).await.unwrap();
    //     }

    //     counter += 1;
    //     println!("counter: {}", counter);
    // }

    // transaction.commit().await.unwrap();

    // ---------------------------------------------------------------------------------------------

    if query.cep.len() < 5 {
        return (
            StatusCode::BAD_REQUEST,
            "CEP deve ter pelo menos 5 caracteres".to_string(),
        );
    }

    let ceps = match query.cep.len() {
        8 => vec![query_one!(transaction, query)],
        _ => query_multiple!(transaction, query),
    };

    let ceps = ceps
        .into_iter()
        .map(|cep| Cep {
            id: cep.id,
            bairro_num: cep.bairro_num,
            cep: cep.cep,
            faixas_caixa_postal: cep
                .caixa_inicial
                .unwrap_or(vec![])
                .iter()
                .zip(cep.caixa_final.unwrap_or(vec![]).iter())
                .map(|(caixa_inicial, caixa_final)| CaixaPostal {
                    caixa_inicial: caixa_inicial.to_string(),
                    caixa_final: caixa_final.to_string(),
                })
                .collect(),
            faixas_cep: cep.faixas_cep,
            localidade_no_sem: cep.localidade_no_sem,
            localidade_num: cep.localidade_num,
            localidade: cep.localidade,
            localidade_subordinada: cep.localidade_subordinada,
            logradouro_dnec: cep.logradouro_dnec,
            logradouro_texto: cep.logradouro_texto,
            nome_unidade: cep.nome_unidade,
            numero_localidade: cep.numero_localidade,
            situacao: cep.situacao,
            tipo_cep: cep.tipo_cep,
            uf: cep.uf,
        })
        .collect::<Vec<Cep>>();

    transaction.commit().await.unwrap();

    (StatusCode::OK, serde_json::to_string_pretty(&ceps).unwrap())
}

#[derive(serde::Deserialize)]
struct QueryValues {
    cep: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CepDb {
    #[serde(skip_serializing)]
    id: i32,

    #[serde(skip_serializing)]
    caixa_inicial: Option<Vec<String>>,

    #[serde(skip_serializing)]
    caixa_final: Option<Vec<String>>,

    #[serde(rename = "baiNu")]
    bairro_num: String,

    cep: String,

    #[serde(rename = "faixasCep")]
    faixas_cep: Vec<String>,

    #[serde(rename = "locNoSem")]
    localidade_no_sem: String,

    #[serde(rename = "locNu")]
    localidade_num: String,

    localidade: String,

    #[serde(rename = "localidadeSubordinada")]
    localidade_subordinada: String,

    #[serde(rename = "logradouroDNEC")]
    logradouro_dnec: String,

    #[serde(rename = "logradouroTexto")]
    logradouro_texto: String,

    #[serde(rename = "nomeUnidade")]
    nome_unidade: String,

    #[serde(rename = "numeroLocalidade")]
    numero_localidade: String,

    situacao: String,

    #[serde(rename = "tipoCep")]
    tipo_cep: String,

    uf: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Cep {
    #[serde(skip_serializing)]
    id: i32,

    #[serde(rename = "baiNu")]
    bairro_num: String,

    cep: String,

    #[serde(rename = "faixasCaixaPostal")]
    faixas_caixa_postal: Vec<CaixaPostal>,

    #[serde(rename = "faixasCep")]
    faixas_cep: Vec<String>,

    #[serde(rename = "locNoSem")]
    localidade_no_sem: String,

    #[serde(rename = "locNu")]
    localidade_num: String,

    localidade: String,

    #[serde(rename = "localidadeSubordinada")]
    localidade_subordinada: String,

    #[serde(rename = "logradouroDNEC")]
    logradouro_dnec: String,

    #[serde(rename = "logradouroTexto")]
    logradouro_texto: String,

    #[serde(rename = "nomeUnidade")]
    nome_unidade: String,

    #[serde(rename = "numeroLocalidade")]
    numero_localidade: String,

    situacao: String,

    #[serde(rename = "tipoCep")]
    tipo_cep: String,

    uf: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CaixaPostal {
    #[serde(rename = "caixaFinal")]
    caixa_final: String,

    #[serde(rename = "caixaInicial")]
    caixa_inicial: String,
}
