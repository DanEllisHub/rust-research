use rocket::{Rocket, Build, futures};
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_db_pools::{Database, Connection};

use futures::{stream::TryStreamExt, future::TryFutureExt};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::outcome::Outcome::Error;

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    text: String,
}

struct UserType {
    user_type: String,
}



// Input 
#[post("/", data = "<post>")]
async fn create(mut db: Connection<Db>, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
        // FP - Not a Result
        let results = sqlx::query!(
            "INSERT INTO posts (title, text) VALUES (?, ?) RETURNING id",
            post.title, post.text
        )
        // Sink
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await?;

    post.id = Some(results.first().expect("returning results").id);
    Ok(Created::new("/").body(post))
}


// Input
#[post("/", data = "<post>")]
async fn create(mut db: Connection<Db>, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    let results = sqlx::query!(
            "INSERT INTO posts (title, text) VALUES (?, ?) RETURNING id",
            // Post contains an ID
            post.id, post.text
        )
        // Sink - Valid Result
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await?;
    post.id = Some(results.first().expect("returning results").id);
    Ok(Created::new("/").body(post))
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

// Input
#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<Post>> {
    sqlx::query!("SELECT id, title, text FROM posts WHERE id = ?", id)
        // Sink
        .fetch_one(&mut **db)
        .map_ok(|r| Json(Post { id: Some(r.id), title: r.title, text: r.text }))
        .await
        .ok()
}

// Input 
#[delete("/<id>")]
async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
        // Sink
        .execute(&mut **db)
        .await?;

    Ok((result.rows_affected() == 1).then_some(()))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserType  {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let token = auth_header.split_whitespace().last().unwrap();
            let secret = "secret"; 

            match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)) {
                Ok(claims) => return Outcome::Success(UserType  { user_type: claims.claims.user_type }),
                Err(_) => return Outcome::Forward(Status::Unauthorized),
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}



#[get("/<id>")]
async fn read_sanitized(mut db: Connection<SqlitePool>, id: i64, user: user_type) -> Option<Json<Post>> {
    //Sanitizer
    sqlx::query!("SELECT id, title, text FROM posts WHERE id = ? AND UserType = ?", id, user.user_type)
        .fetch_one(&mut **db)
        .map_ok(|r| Json(Post { id: Some(r.id), title: r.title, text: r.text }))
        .await
        .ok()
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
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/sqlx", routes![list, create, read, delete, read_sanitized])
    })
}
