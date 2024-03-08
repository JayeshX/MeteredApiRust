
use rocket::{futures::stream::Scan, response, serde::json::Json};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Error, Surreal};

pub struct FreeDatabase {
    pub fdb: Surreal<Client>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreeTrial {
    followers: u32,
    following: u32,
    posts:u32,
    username:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Numcol{
    username:String,
    ncol: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SCol{
    username:String,
    scol:String
}






impl FreeDatabase{
    pub async fn new() -> Result<Self, Error> {
        let fdb = Surreal::new::<Ws>("127.0.0.1:8001").await?;
        fdb.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        fdb.use_ns("todo").use_db("todo").await?;
        
        Ok(FreeDatabase{
            fdb,
        })
    }

    pub async fn finfo(&self) -> Result<Json<Vec<FreeTrial>>, response::status::Custom<String>> {
        
        let query = format!("select followers,following,posts,username from instagram limit 25;");
        let mut response = self.fdb.query(query).await.unwrap();
        let users: Vec<FreeTrial> = response.take(0).unwrap();
        Ok(Json(users))
    }

    pub async fn f_num_info(&self,col:String) -> Result<Json<Vec<(std::string::String, u32)>>, response::status::Custom<String>> {
        // let mut vs = vector
        let query = format!("select username,{col} from instagram limit 25;");
        let mut response = self.fdb.query(query).await.unwrap();
        let users:Vec<(String, u32)>= response.take(0).unwrap();
        // let users: Vec<Numcol> = response.take(0).unwrap();
        Ok(Json(users))
    }

    pub async fn f_string_info(&self,col:String) -> Result<Json<Vec<SCol>>, response::status::Custom<String>> {
        
        let query = format!("select username, {col} from instagram limit 25;");
        let mut response = self.fdb.query(query).await.unwrap();
        let users: Vec<SCol> = response.take(0).unwrap();
        Ok(Json(users))
    }



    // pub async fn finfo(&self) -> Result<Json<Vec<freetrial>>, response::status::Custom<String>> {
    //     let query = format!("select followers, following from instagram;");
    //     let mut response = self.fdb.query(query).await.unwrap();
    //     let users: Vec<freetrial> = response.take(0).unwrap();
        
    //     // Serialize the response to JSON and check its size
    //     let json_response = serde_json::to_string(&users).unwrap();
    //     let response_size = json_response.len();

    //     // If response size exceeds 1KB, return an error
    //     if response_size > 1024 {
    //         return Err(response::status::Custom(
    //             rocket::http::Status::InternalServerError,
    //             "Response size exceeds 1KB".to_string(),
    //         ));
    //     }

    //     Ok(Json(users))
    // }
}
