use serde::{Deserialize};

#[derive(Deserialize)]
struct APIResponse {
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

#[derive(Deserailize)]
struct Links {
	first: String,
	last: String
}