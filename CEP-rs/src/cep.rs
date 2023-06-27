use axum::extract::{Path, State};
use roaring::RoaringBitmap;

pub async fn find_cep(State(ceps_bitmap): State<RoaringBitmap>, Path(cep): Path<String>) -> String {
    if cep.len() != 8 {
        dbg!(cep);
        return "Invalid CEP".to_string();
    }

    let cep_number = match cep.parse::<u32>() {
        Ok(cep_number) => cep_number,
        Err(_) => return "Invalid CEP".to_string(),
    };
    let found = ceps_bitmap.contains(cep_number);

    return match found {
        true => "Found".to_string(),
        false => "Not Found".to_string(),
    };
}
