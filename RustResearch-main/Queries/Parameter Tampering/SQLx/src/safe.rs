use rocket::{Rocket, Build, futures};
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_db_pools::{Database, Connection};
use futures::{stream::TryStreamExt, future::TryFutureExt};
use jsonwebtoken::{encode, decode, EncodingKey, Algorithm, Header, Validation}; 
use jsonwebtoken::errors::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::DecodingKey;
use chrono::Utc;
use std::env; 
use dotenvy::dotenv;
use rocket::request::{Outcome, Request, FromRequest}; 
use rocket::http::Status; 

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    text: String,
    userid: String,
}


pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();
    
    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    }; 

    let header = Header::new(Algorithm::HS512);

    // 👇 New!
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    // 👇 New!
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}


#[get("/login")]
pub fn login_user_handler() -> Result<String, NetworkResponse> {
    //Implement logic here .... not complete
    let token = create_jwt(1).unwrap();
    let response = Response {body: ResponseBody::AuthToken(token)};
    Ok(serde_json::to_string(&response).unwrap())
}



#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    let ids = sqlx::query!("SELECT id FROM posts")
        .fetch(&mut **db)
        .map_ok(|record| record.id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(ids))
}

// 👇 New!
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        // check if the token passed in is valid or not.
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }  
        // getting the token from the users request
        //  search for the “Authorization” header in the request.
        /* 
        Check Authorization header:
            - None - 401 Unauthorized ("Error validating JWT - No token provided")
            - Some(key) - validate key using is_valid()
            - Ok(claims) - return Outcome::Success(JWT {claims})
            - Err(err) - match against the err.kind()
            - ErrorKind::ExpiredSignature - 401 Unauthorized ("Error validating JWT - Expired Token")
            - ErrorKind::InvalidToken - 401 Unauthorized ("Error validating JWT - Invalid Token")
            - Anything else - 401 Unauthorized ("Error validating JWT - {err}")
        
         */
        match req.headers().get_one("authorization") {
            None => {
                let response = Response { 
                    body: ResponseBody::Message(
                        String::from("Error validating JWT token - No token provided")
                    )
                };

                Outcome::Error((
                    Status::Unauthorized, 
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                )) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { 
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Expired Token")
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Invalid Token")
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    _ => {
                        let response = Response { 
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - {}", err)
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized, 
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    }
                }
            },
        }  
    }
}

#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64, key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    let userid = key?.claims.subject_id;
    // Sanitizer 
    let result = sqlx::query!("SELECT id, title, text, userid FROM posts WHERE id = ? and userid = ?", id, userid)
        .fetch_one(&mut **db)
        .map_ok(|r| Json(Post { id: Some(r.id), title: r.title, text: r.text, userid: r.userid}))
        .await;

    let response = Response {
        body: ResponseBody::Message(format!("Post: {:?}", result)),
    };
    Ok(serde_json::to_string(&response).unwrap())
}



async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    dotenv().ok();

    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/sqlx", routes![list, create, read, delete, destroy,login_user_handler])
    })
}
