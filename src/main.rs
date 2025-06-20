use actix_web::web::Data;
use actix_web::{body, web::Path, get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use uuid::Uuid;
use validator::Validate;
use crate::db::{pizza_data_trait::PizzaDataTrait, Database};
use crate::error::PizzaError;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL, Pizza};
mod models;
mod db;
mod error;

#[get("/pizzas")]
async fn get_pizza(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {  //changed the retun type from impl Responder
    // HttpResponse::Ok().body("pizzas available")
    let pizzas = db.get_all_pizzas(&db).await;

    match pizzas {
        // Some(found_pizza) => HttpResponse::Ok().body(format!("{:?}", found_pizza)),
        // None => HttpResponse::Ok().body("Error!"),

        Some(found_pizza) => Ok(Json(found_pizza)),
        None => Err(PizzaError::NoPizzasFound),
    }
}



#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {  //changed the retun type from impl Responder

   // change let is_valid = body.validate() 
    match body.validate() {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = db.add_pizza(&db, Pizza::new(
                String::from(new_uuid),
                pizza_name,
            ))
            .await;

            match new_pizza {
                Some(created) => Ok(Json(created)),
                None => Err(PizzaError::PizzaCreationfailure),
            }

            // HttpResponse::Ok().body(format!("pizza entered is {pizza_name}"))
        },
        Err(_) => Err(PizzaError::PizzaCreationfailure),
    }

}



#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = db.update_pizza(&db, uuid).await;

    match update_result {
       Some(updated_pizza) => Ok(Json(updated_pizza)),
       None => Err(PizzaError::NoSuchPizzaFound)
    }

    // HttpResponse::Ok().body(format!("Updating the Pizza with {uuid}"))
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await 
        .expect("Error connecting to database");

    let db_data = Data::new(db);
    
    HttpServer::new( move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizza)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
