
use rocket::form::Form;
use rocket::{response, State};
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod database;
use database::{trial1, APIKey, Database, PersonWithId, X1};
#[macro_use]
extern crate rocket;

#[get("/")]
fn world() -> &'static str {
    "hello world"
}


#[get("/intr")]
async fn instr(db: &State<Database>) -> Result<Json<Vec<trial1>>, response::status::Custom<String>> {
    let x = db.instrial().await;
    x
}

#[get("/t?<api_key>")]
async fn sel(db: &State<Database>, api_key: String) -> Result<Json<Vec<PersonWithId>>, response::status::Custom<String>> {
    let x = db.getf(&api_key).await;
    x
}

#[get("/getusers?<api_key>")]
async fn sel1(db: &State<Database>, api_key: String) -> Result<Json<Vec<X1>>, response::status::Custom<String>> {
    let x = db.getx(&api_key).await;
    x
}

#[get("/getbyid?<api_key>&<idx>")]
async fn getuserbyid(db: &State<Database>, api_key: String, idx: String) -> Result<Json<Vec<X1>>, response::status::Custom<String>> {
    let x = db.user_from_id(&api_key, idx).await;
    x
}

#[post("/createkey?<user_id>")]
async fn create_api_key(db: &State<Database>, user_id: String) -> Json<String> {
    // let key = db.create_api_key(user_id);
    // Json(key)
    // let x:String = ;
    match db.create_api_key(user_id).await{
    Ok(key) => rocket::serde::json::Json(key),
    Err(err) => rocket::serde::json::Json(String::from("key creation failed")),
    }
        
}


#[delete("/deletekey?<api_key>")]
async fn delete_api_key(db: &State<Database>, api_key: String) -> Json<String> {
    db.delete_api_key(&api_key);
    Json(format!("API key {} deleted", api_key))
}
#[derive(FromForm)]
struct CreateAccount {
    name: String,
    email:String,
}


#[post("/user",data="<account>")]  
async fn new_user(db:&State<Database>,account:Form<CreateAccount>)->Result<response::content::RawJson<String>, response::status::Custom<String>>{
    match db.new_user(account.name.clone(),account.email.clone()).await{
        Ok(_) => Ok(response::content::RawJson(format!("table create"))),
        // Ok(_) => Ok(response::content::Json(format!("Table created successfully"))),
        Err(err) => Err(response::status::Custom(rocket::http::Status::InternalServerError, format!("Failed to create table: {}", err))),
    
    }
    
}


//free routes
#[get("/info")]
async fn freeinfo(fdb:&State<FreeDatabase>)->Result<Json<Vec<FreeTrial>>, response::status::Custom<String>> {
    let x = fdb.finfo().await;
    x
}

// #[get("/col?<colname>")]
// async fn colhandler(fdb: &State<FreeDatabase>, colname: String) {
//     if colname=="post"||colname=="followers"||colname=="following"||colname=="bio_length"{
//         getnumcoldata(fdb,colname).await.unwrap();
//     }else if colname=="account_type" {
//         getstringcoldata(fdb,colname).await.unwrap();
//     }else{
//         ereturn();
//     }
    
// }
// fn ereturn()-> response::status::Custom<String> {
//     response::status::Custom(rocket::http::Status::InternalServerError, format!("Failed to find column"))
// }

// async fn getnumcoldata(fdb:&State<FreeDatabase>,colname:String)->Result<Json<Vec<Numcol>>, response::status::Custom<String>> {
//     let x = fdb.f_num_info(colname).await;
//     x
// }
// async fn getstringcoldata(fdb:&State<FreeDatabase>,colname:String)->Result<Json<Vec<SCol>>, response::status::Custom<String>> {
//     let x = fdb.f_string_info(colname).await;
//     x
// }
#[get("/followers")]
async fn followerslist(fdb:&State<FreeDatabase>)->Result<Json<Vec<(std::string::String, u32)>>, response::status::Custom<String>> {
        let x = fdb.f_num_info(String::from("followers")).await;
        x
    }

mod freetier;
use freetier::{FreeDatabase, FreeTrial, Numcol, SCol};
#[launch]
async fn rocket() -> _ {
    let db = Database::new().await.unwrap();
    // let fdb = freetier::FreeDatabase::new().await.unwrap();
    let fdb   = FreeDatabase::new().await.unwrap();
    rocket::build()
        .manage(db)
        .manage(fdb)
        .mount("/", routes![world, sel, sel1, getuserbyid,instr])
        .mount("/apikeys", routes![create_api_key, delete_api_key])
        .mount("/new", routes![new_user])
        .mount("/free",routes![freeinfo])
        .mount("/free/col", routes![followerslist])
}
