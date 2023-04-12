use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct APIResponse<T> {
    pub meta: ApiMetadata,
    pub json: JsonApiVersion,
    pub links: Links,
    pub data: Vec<T>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ApiMetadata {
    pub page: PageData,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PageData {
    pub currentPage: i32,
    pub from: i32,
    pub lastPage: i32,
    pub perPage: i32,
    pub to: i32,
    pub total: i32,
}

#[derive(Deserialize)]
pub struct JsonApiVersion {
    pub version: String,
}

#[derive(Deserialize)]
pub struct Links {
    pub first: String,
    pub last: String,
}
