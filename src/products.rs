use crate::client::{Client, RequestType};
use reqwest::Result as ReqwestResult;
use serde::{Deserialize};

struct Products;

#[derive(Deserialize)]
pub struct Product {
	r#type: String,
	id: String,
	productinfo: ProductInfo
}

#[derive(Deserialize)]
pub struct ProductInfo {
	store_id: i32,
	name: String,
	slug: String,
	description: String,
	status: String,
	status_formatted: String,
	thumb_url: String,
	large_thumb_url: String,
	price: i32,
	pay_what_you_want: bool,
	from_price: Option<f32>,
	to_price: Option<f32>,
	buy_now_url: String,
	price_formatted: Option<String>,
	created_at: String,
	updated_at: String
}

enum ProductLinks {
	GetAllProducts,
	GetOneProduct
}

impl ProductLinks {
	fn as_str(&self, id: Option<i32>) -> Result<String, Box<dyn std::error::Error>> {
		match self {
			ProductLinks::GetAllProducts => {Ok("https://api.lemonsqueezy.com/v1/products".to_string())},
			ProductLinks::GetOneProduct => {

				let Some(id) = id else {
					return Err("None (or invalid) ID inputted".into())
				};

					Ok(format!("https://api.lemonsqueezy.com/v1/products/{id}"))
				}
			}
		}
}

impl Products {
	pub async fn get_all_products(ctx: Client) -> ReqwestResult<Product> {

		let url = ProductLinks::GetAllProducts.as_str(None).unwrap();
		
		let req = ctx.send_req(url, RequestType::Get);
		
		let res = req.await?;

		match res.json::<Product>().await {
			Ok(res) => Ok(res),
			Err(err) => Err(err)
		}
	}

	pub async fn get_one_product(ctx: Client, id: i32) -> ReqwestResult<Product> {

		let url = ProductLinks::GetOneProduct.as_str(id).unwrap();

		let req = ctx.send_req(url, RequestType::Get);
		
		let res = req.await?;

		match res.json::<Product>().await {
			Ok(res) => Ok(res),
			Err(err) => Err(err)
		}
		
	}
}
