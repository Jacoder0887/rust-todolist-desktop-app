use std::collections::HashMap;
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize}; 
use futures::future::join_all;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    name: String,
    description: String,
    price: f32,
    qty: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String
}


async  fn create_Todo(firebase_client: &Firebase, Todo: &Todo)
    -> Response {
        let  firebase = firebase_client.at("Todos");
        let Todo_res = firebase.set::<Todo>(&Todo)
        .await;

    string_to_res(&Todo_res.unwrap().data) 
}

async fn get_Todos(firebase_client: &Firebase) -> HashMap<String,Todo>{
    let  firebase = firebase_client.at("Todos");
    let Todo = firebase.get::<HashMap<String,Todo>>()
        .await.unwrap();
    Todo
}
 
async fn get_Todos_by_id(firebase_client: &Firebase, id: &String ) -> HashMap<String,Todo>{
    let  firebase = firebase_client.at("Todos").at(&id);
    let Todo = firebase.get::<HashMap<String,Todo>>()
        .await.unwrap();
    Todo
}

fn string_to_res(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

#[tokio::main]
async fn main() {
    let firebase = Firebase::new("https://jasperlog-c7333.firebaseio.com/")
    .unwrap();

    let mut prod = vec![
        Todo {
            name: String::from("Nike"),
            description: String::from("Awesome"),
            price: 200.23,
            qty: 111
        },
        Todo {
            name: String::from("sniper"),
            description: String::from("Awesome"),
            price: 200.23,
            qty: 111
        },
        Todo {
            name: String::from("m16A1"),
            description: String::from("assult rifle"),
            price: 202340.23,
            qty: 23
        },
         Todo {
            name: String::from("M60"),
            description: String::from("machine gun"),
            price: 340.23,
            qty: 13
        },
         Todo {
            name: String::from("AKK47"),
            description: String::from("assult rifle"),
            price: 130.23,
            qty: 111
        }
    ];

    let result: Vec<_> = prod.iter()
        .map(|p| create_Todo(&firebase, &p))
        .collect();

    let res = join_all(result).await;    
    // let  result = create_Todo(&firebase, &prod)
    //     .await;
    
    println!("Successfully created to DB! {:?}", res);

    let Todos = get_Todos(&firebase).await;

    println!("Prroducts in DB {:?}", Todos);

   // get_Todos_by_id = get_Todos_by_id(&firebase, String::from("")).await;
}

