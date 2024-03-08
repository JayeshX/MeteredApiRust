use rocket::{response, State};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonWithId {
    id: Thing,
    x1: String,
    x2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct trial1 {
    followers: u32,
    following: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct X1 { 
    name: String,
    occ: String,
}

#[derive(Debug, Clone)]
pub struct APIKey {
    pub key: String,
    pub user_id: String,
    pub usage_count: usize,
}

pub struct Database {
    pub db: Surreal<Client>,
    pub api_keys: Arc<Mutex<HashMap<String, APIKey>>>,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("todo").use_db("todo").await?;
        Ok(Database {
            db,
            api_keys: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    pub async fn instrial(&self) -> Result<Json<Vec<trial1>>, response::status::Custom<String>> {
        
        let query = format!("select followers,following from instagram;");
        let mut response = self.db.query(query).await.unwrap();
        let users: Vec<trial1> = response.take(0).unwrap();
        Ok(Json(users))
    }

    pub async fn create_api_key(&self, user_id: String) -> Result<String,Error> {
        let key = generate_unique_key();
        let api_key = APIKey {
            key: key.clone(),
            user_id: user_id.clone(), // Associate the user ID with the API key
            usage_count: 0,
        };
        self.api_keys.lock().unwrap().insert(key.clone(), api_key);
        // update users set keys+=["rust"] where name = "jay";
        self.db.query(format!(r#"update users set keys+=["{key}"] where name = "{user_id}";insert into apikeys (key,reqc,price) values ("{key}",0,0);"#)).await?;
        Ok(key)
    }
    pub fn delete_api_key(&self, key: &str) {
        self.api_keys.lock().unwrap().remove(key);
    }
    pub async fn getf(&self, api_key: &str) -> Result<Json<Vec<PersonWithId>>, response::status::Custom<String>> {
        if !self.validate_api_key(api_key) {
            return Err(response::status::Custom(rocket::http::Status::Unauthorized, "Invalid API key".into()));
        }
        let query = format!("SELECT * from reading;");
        let mut response = self.db.query(query).await.unwrap();
        let users: Vec<PersonWithId> = response.take(0).unwrap();
        Ok(Json(users))
    }

    pub async fn getx(&self, api_key: &str) -> Result<Json<Vec<X1>>, response::status::Custom<String>> {
        if !self.validate_api_key(api_key) {
            return Err(response::status::Custom(rocket::http::Status::Unauthorized, "Invalid API key".into()));
        }
        let query = format!("SELECT * from reading;");
        let mut response = self.db.query(query).await.unwrap();
        let user: Vec<X1> = response.take(0).unwrap();
        Ok(Json(user))
    }

    pub async fn user_from_id(&self, api_key: &str, id: String) -> Result<Json<Vec<X1>>, response::status::Custom<String>> {
        if !self.validate_api_key(api_key) {
            return Err(response::status::Custom(rocket::http::Status::Unauthorized, "Invalid API key".into()));
        }
        let query = format!("select name, occ from reading where id={id};");
        let mut response = self.db.query(query).await.unwrap();
        let user: Vec<X1> = response.take(0).unwrap();
        Ok(Json(user))
    }

    pub async fn new_user(&self,name:String,email:String)->Result<(),Error>{
        
        self.db.query(format!(r#"INSERT INTO users (name,email,keys,bill) values ("{name}","{email}",[],"0")"#)).await?;
        Ok(())
    }


    fn validate_api_key(&self, api_key: &str) -> bool {
        self.api_keys.lock().unwrap().contains_key(api_key)
    }

    // Implement other methods like getf(), getx(), user_from_id() here...
}

// Utility function to generate a unique API key
fn generate_unique_key() -> String {
    // Implement your logic to generate a unique key here (e.g., UUID)
    "rust3".to_string()
}

