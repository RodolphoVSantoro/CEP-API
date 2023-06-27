#[macro_export]
macro_rules! query_one {
    ($transaction:expr, $query:expr) => {
        sqlx::query_as!(
            CepDb,
            "SELECT c.* ,
            (SELECT array(SELECT f.caixa_inicial FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_inicial,
            (SELECT array(SELECT f.caixa_final FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_final
            FROM ceps c WHERE c.cep = $1",
            $query.cep
        ).fetch_one(&mut $transaction).await.unwrap()
    }
}

#[macro_export]
macro_rules! query_multiple {
    ($transaction:expr, $query:expr) => {
        sqlx::query_as!(
            CepDb,
            "SELECT c.* ,
            (SELECT array(SELECT f.caixa_inicial FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_inicial,
            (SELECT array(SELECT f.caixa_final FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_final
            FROM ceps c WHERE c.cep LIKE $1",
            format!("{}%", $query.cep)
        ).fetch_all(&mut $transaction).await.unwrap()
    }
}
