use crate::client::{Client, RequestType};
use crate::json::APIResponse;
use reqwest::Result as ReqwestResult;
use serde::Deserialize;

pub struct Products;

#[derive(Deserialize)]
pub struct Product {
    pub r#type: String,
    pub id: String,
    pub productinfo: ProductInfo,
}

#[derive(Deserialize)]
pub struct ProductInfo {
    pub store_id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub status: String,
    pub status_formatted: String,
    pub thumb_url: String,
    pub large_thumb_url: String,
    pub price: i32,
    pub pay_what_you_want: bool,
    pub from_price: Option<f32>,
    pub to_price: Option<f32>,
    pub buy_now_url: String,
    pub price_formatted: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

enum ProductLinks {
    GetAllProducts,
    GetOneProduct,
}

impl ProductLinks {
    fn as_str(&self, id: Option<i32>) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            ProductLinks::GetAllProducts => {
                Ok("https://api.lemonsqueezy.com/v1/products".to_string())
            }
            ProductLinks::GetOneProduct => {
                let Some(item) = id else {
					return Err("None (or invalid) ID inputted".into())
				};

                Ok(format!("https://api.lemonsqueezy.com/v1/products/{item}"))
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
            Err(err) => Err(err),
        }
    }

    pub async fn get_one_product(ctx: Client, id: i32) -> ReqwestResult<APIResponse<Product>> {
        let url = ProductLinks::GetOneProduct.as_str(Some(id)).unwrap();

        let req = ctx.send_req(url, RequestType::Get);

        let res = req.await?;

        match res.json::<APIResponse<Product>>().await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}
