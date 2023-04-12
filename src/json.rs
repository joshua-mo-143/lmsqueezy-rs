use serde::{Deserialize};

#[derive(Deserialize)]
pub struct APIResponse<T> {
	meta: ApiMetadata,
	jsonapi: JsonApiVersion,
	links: Links,
	data: Vec<T>
	
}

#[derive(Deserialize)]
struct ApiMetadata {
	page: PageData
}

#[derive(Deserialize)]
struct PageData {
	currentPage: i32,
	from: i32,
	lastPage: i32,
	perPage: i32,
	to: i32,
	total: i32
}

#[derive(Deserialize)]
struct JsonApiVersion {
	version: String
}

#[derive(Deserialize)]
struct Links {
	first: String,
	last: String
}