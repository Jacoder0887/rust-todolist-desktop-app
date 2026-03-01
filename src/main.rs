use std::collections::HashMap;
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize}; 
use futures::future::join_all;

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    name: String,
    description: String,
    price: f32,
    qty: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String
}


async  fn create_product(firebase_client: &Firebase, product: &Product)
    -> Response {
        let  firebase = firebase_client.at("products");
        let product_res = firebase.set::<Product>(&product)
        .await;

    string_to_res(&product_res.unwrap().data) 
}

async fn get_products(firebase_client: &Firebase) -> HashMap<String,Product>{
    let  firebase = firebase_client.at("products");
    let product = firebase.get::<HashMap<String,Product>>()
        .await.unwrap();
    product
}
 
async fn get_products_by_id(firebase_client: &Firebase, id: &String ) -> HashMap<String,Product>{
    let  firebase = firebase_client.at("products").at(&id);
    let product = firebase.get::<HashMap<String,Product>>()
        .await.unwrap();
    product
}

fn string_to_res(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

#[tokio::main]
async fn main() {
    let firebase = Firebase::new("https://jasperlog-c7333.firebaseio.com/")
    .unwrap();

    let mut prod = vec![
        Product {
            name: String::from("Nike"),
            description: String::from("Awesome"),
            price: 200.23,
            qty: 111
        },
        Product {
            name: String::from("m16A1"),
            description: String::from("assult rifle"),
            price: 202340.23,
            qty: 23
        },
         Product {
            name: String::from("M60"),
            description: String::from("machine gun"),
            price: 340.23,
            qty: 13
        },
         Product {
            name: String::from("AKK47"),
            description: String::from("assult rifle"),
            price: 130.23,
            qty: 111
        }
    ];

    let result: Vec<_> = prod.iter()
        .map(|p| create_product(&firebase, &p))
        .collect();

    let res = join_all(result).await;    
    // let  result = create_product(&firebase, &prod)
    //     .await;
    
    println!("Successfully created to DB! {:?}", res);

    let products = get_products(&firebase).await;

    println!("Prroducts in DB {:?}", products);

   // get_products_by_id = get_products_by_id(&firebase, String::from("")).await;
}

