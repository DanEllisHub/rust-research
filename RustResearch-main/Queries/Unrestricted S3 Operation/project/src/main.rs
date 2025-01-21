#[macro_use]
extern crate rocket;
use aws_sdk_s3 as s3;
use aws_sdk_s3control as s3control;
use s3::primitives::ByteStream;
use std::fs;

// bucket = "ori-public-bucket-rust";
// key = "test.txt";

static PATH: &str = "../../file-body.txt";

#[get("/s3?<bucket>&<key>&<t>&<s>")]
async fn s3endpoint(bucket: &str, key: &str, t: u16, s: u16) -> String {
    let config = aws_config::load_from_env().await;

    let client = aws_sdk_s3::Client::new(&config);
    let control_client = s3control::Client::new(&config);
    ////////////////////////////////////////////
    // IMPORTANT NOTES:
    //////////////////////////////////////////
    // `bucket`, `key`, `t` and `s` are interactive inputs.
    // for the queries unrestricted_read/write/delete, only `bucket` and `key` are considered
    // the value of `t` will dynamically defines the type of vulnerability, e.g.:
    //          - unrestricted delete
    //          - unrestricted write
    //          - unrestricted read
    // the value of `s` will dynamically defines the type of the sanitizer, e.g.:
    //          - tagging
    //          - metadata
    //          - etc.
    ///////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////

    match t {
        1 => remove_object_or_bucket(&client, &control_client, bucket, key)
            .await
            .expect("remove_object failed"),
        2 => remove_object_sanitized(&client, bucket, key, s)
            .await
            .expect("remove_object_sanitized failed"),
        3 => read_object(&client, bucket, key)
            .await
            .expect("read_object failed"),
        4 => read_object_sanitized(&client, bucket, key, s)
            .await
            .expect("read_object_sanitized failed"),
        5 => write_object(&client, bucket, key)
            .await
            .expect("write_object failed"),
        6 => write_object_sanitized(&client, bucket, key, s)
            .await
            .expect("write_object failed"),
        7 => another_safe_read(&client, bucket, key)
            .await
            .expect("something went wrong"),
        _ => "unkown operation".to_string(),
    }
}

async fn remove_object_or_bucket(
    client: &s3::Client,
    ctrl_client: &s3control::Client,
    bucket: &str,
    key: &str,
) -> Result<String, s3::Error> {
    // RESULT 1: Unrestricted_delete
    client
        .delete_object()
        .bucket(bucket)
        .key(key) // RESULT - unrestricted_delete
        .send()
        .await?;

    // RESULT 2: Unrestricted_delete
    client
        .delete_bucket()
        .bucket(bucket) // RESULT - unrestricted_delete
        .send()
        .await?;

    // RESULT 3: Unrestricted_delete
    let mut delete_objects: Vec<s3::types::ObjectIdentifier> = vec![];
    let object_to_delete = s3::types::ObjectIdentifier::builder()
        .set_key(Some(key.to_string())) // `key` (input) influences the key of the object to be deleted
        .build()
        .expect("something went wrong");
    delete_objects.push(object_to_delete);
    client
        .delete_objects()
        .bucket(bucket)
        .delete(
            s3::types::Delete::builder()
                .set_objects(Some(delete_objects)) // RESULT
                .build()
                .expect("something went wrong with removing multiple objects"),
        )
        .send()
        .await?;

    // RESULT 4 - Unrestricted_Delete
    _ = ctrl_client
        .delete_bucket()
        .bucket(bucket) // RESULT
        .send()
        .await;

    println!("remove_object");

    Ok("done".to_string())
}

async fn remove_object_sanitized(
    client: &s3::Client,
    bucket: &str,
    key: &str,
    sanitizer_index: u16,
) -> Result<String, s3::Error> {
    match sanitizer_index {
        1 => {
            if sanitizer_1(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .delete_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED - unrestricted_delete
                    .send()
                    .await?;
            }
        }
        2 => {
            if sanitizer_2(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .delete_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED - unrestricted_delete
                    .send()
                    .await?;
            }
        }
        3 => {
            if sanitizer_3(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .delete_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED - unrestricted_delete
                    .send()
                    .await?;
            }
        }
        _ => {
            println!("sanitizer not found");
        }
    }

    Ok("done".to_string())
}

async fn read_object(client: &s3::Client, bucket: &str, key: &str) -> Result<String, s3::Error> {
    let obj = client
        .get_object()
        .bucket(bucket)
        .key(key) // RESULT
        .send()
        .await?;
    let bytes = obj
        .body
        .collect()
        .await
        .expect("error in reading object's body")
        .into_bytes();
    let response = std::str::from_utf8(&bytes).expect("error in converting bytes to string");

    return Ok(response.to_string());
}

async fn read_object_sanitized(
    client: &s3::Client,
    bucket: &str,
    key: &str,
    sanitizer_index: u16,
) -> Result<String, s3::Error> {
    match sanitizer_index {
        1 => {
            if sanitizer_1(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                let obj = client
                    .get_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED
                    .send()
                    .await?;
                let bytes = obj
                    .body
                    .collect()
                    .await
                    .expect("error in reading object's body")
                    .into_bytes();
                let response =
                    std::str::from_utf8(&bytes).expect("error in converting bytes to string");
                return Ok(response.to_string());
            }
        }
        2 => {
            if sanitizer_2(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                let obj = client
                    .get_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED
                    .send()
                    .await?;

                let bytes = obj
                    .body
                    .collect()
                    .await
                    .expect("error in reading object's body")
                    .into_bytes();
                let response =
                    std::str::from_utf8(&bytes).expect("error in converting bytes to string");
                return Ok(response.to_string());
            }
        }
        3 => {
            if sanitizer_3(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                let obj = client
                    .get_object()
                    .bucket(bucket)
                    .key(key) // SANITIZED
                    .send()
                    .await?;
                let bytes = obj
                    .body
                    .collect()
                    .await
                    .expect("error in reading object's body")
                    .into_bytes();
                let response =
                    std::str::from_utf8(&bytes).expect("error in converting bytes to string");
                return Ok(response.to_string());
            }
        }
        _ => {
            println!("sanitizer not found");
        }
    }

    Ok("done".to_string())
}

async fn write_object(client: &s3::Client, bucket: &str, key: &str) -> Result<String, s3::Error> {
    use s3::primitives::ByteStream;
    let body: ByteStream = ByteStream::from_path(std::path::Path::new(&PATH.clone()))
        .await
        .expect("something went wrong");
    let body1: ByteStream = ByteStream::from_path(std::path::Path::new(&PATH.clone()))
        .await
        .expect("something went wrong");
    let body2: ByteStream = ByteStream::from_path(std::path::Path::new(&PATH.clone()))
        .await
        .expect("something went wrong");

    // Example #1:
     client
        .put_object()
        .bucket(bucket)
        .key(key) // RESULT
        .body(body)
        .send()
        .await?;

    // // Example #2:
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body1)
        .send()
        .await?;

    // Example #3:
    multipart(key, bucket, body2, client).await;
    println!("write_object");

    Ok("done".to_string())
}

async fn multipart(key: &str, bucket: &str, body: ByteStream, client: &s3::Client) {
    println!("in multipart");
    println!("key: {}", key);
    println!("bucket: {}", bucket);

    let multipart_upload_res: s3::operation::create_multipart_upload::CreateMultipartUploadOutput =
        client
            .create_multipart_upload()
            .bucket(bucket)
            .key(key) // RESULT
            .send()
            .await
            .unwrap();
    let upload_id = multipart_upload_res.upload_id().unwrap();
    let mut upload_parts: Vec<s3::types::CompletedPart> = Vec::new();

    //Chunk index needs to start at 0, but part numbers start at 1.
    let upload_part_res = client
        .upload_part()
        .key(key) // RESULT
        .bucket(bucket)
        .upload_id(upload_id)
        .body(body)
        .part_number(1)
        .send()
        .await
        .expect("msg");
    upload_parts.push(
        s3::types::CompletedPart::builder()
            .e_tag(upload_part_res.e_tag.unwrap_or_default())
            .part_number(1)
            .build(),
    );

    let completed_multipart_upload: s3::types::CompletedMultipartUpload =
        s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(upload_parts))
            .build();

    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
}

async fn write_object_sanitized(
    client: &s3::Client,
    bucket: &str,
    key: &str,
    sanitizer_index: u16,
) -> Result<String, s3::Error> {
    use s3::primitives::ByteStream;
    let body: ByteStream = ByteStream::from_path(std::path::Path::new(&PATH.clone()))
        .await
        .expect("something went wrong");

    match sanitizer_index {
        1 => {
            if sanitizer_1(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .put_object()
                    .bucket(bucket)
                    .key(key) // RESULT
                    .body(body)
                    .send()
                    .await?;
                return Ok("ok".to_string());
            }
        }
        2 => {
            if sanitizer_2(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .put_object()
                    .bucket(bucket)
                    .key(key) // RESULT
                    .body(body)
                    .send()
                    .await?;
                return Ok("ok".to_string());
            }
        }
        3 => {
            if sanitizer_3(client, bucket, key)
                .await
                .expect("santizer failed")
            {
                client
                    .put_object()
                    .bucket(bucket)
                    .key(key) // RESULT
                    .body(body)
                    .send()
                    .await?;
                return Ok("ok".to_string());
            }
        }
        _ => {
            println!("sanitizer not found");
        }
    }

    Ok("done".to_string())
}

fn get_user_id() -> String {
    // fake authentication method to get the current logged in user
    "some-user-id".to_string()
}

async fn another_safe_read(client: &s3::Client, bucket: &str, key: &str) -> Result<String, s3::Error> {
    let obj = client
        .get_object()
        .bucket(bucket)
        .key(key) // RESULT
        .send()
        .await?;
    let bytes = obj
        .body
        .collect()
        .await
        .expect("error in reading object's body")
        .into_bytes();

    let metadata = obj
        .metadata
        .expect("sanitizer_2 could not read the metadata of the object");
    let user_id = metadata
        .get("user_id")
        .expect("sanitizer_2 could not read the user_id metadata");
    let mut response: &str;
    if user_id.eq(&get_user_id()) {
        response = std::str::from_utf8(&bytes).expect("error in converting bytes to string");
    } else {
        response = std::str::from_utf8(&bytes).expect("error in converting bytes to string");
    }
    return Ok(response.to_string());
}


async fn sanitizer_1(client: &s3::Client, bucket: &str, key: &str) -> Result<bool, s3::Error> {
    let head = client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect("sanitizer_1 could not read the head");
    let metadata = head
        .metadata()
        .expect("sanitizer_1 could not read the metadata from the head");
    let user_id = metadata
        .get("user_id")
        .expect("sanitizer_1 could not read user_id value from metadata");
    return Ok(user_id.eq(&get_user_id()));
}

async fn sanitizer_2(client: &s3::Client, bucket: &str, key: &str) -> Result<bool, s3::Error> {
    let obj = client.get_object().bucket(bucket).key(key).send().await?;
    let metadata = obj
        .metadata()
        .expect("sanitizer_2 could not read the metadata of the object");
    let user_id = metadata
        .get("user_id")
        .expect("sanitizer_2 could not read the user_id metadata");
    return Ok(user_id.eq(&get_user_id()));
}

async fn sanitizer_3(client: &s3::Client, bucket: &str, key: &str) -> Result<bool, s3::Error> {
    println!("sanitizer 3 started");
    let tags = client
        .get_object_tagging()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    let t = tags.tag_set();
    let sa = t
        .iter()
        .find(|&x| x.key == "user_id")
        .expect("no tag with user_id key was found");
    let n = sa.value.clone();
    return Ok(n.as_str().eq(&get_user_id()));
}

fn get_stored_input() -> String {
    let path = "stored.txt".to_string();

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    return contents;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![s3endpoint])
}
