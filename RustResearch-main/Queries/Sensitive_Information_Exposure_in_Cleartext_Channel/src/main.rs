mod reqwest_example;
use std::{io::{stdout, Read, Write}, sync::Arc};
use rustls::RootCertStore;
use std::process::Command;

#[tokio::main]
async fn main() {
    reqwest_example::reqwest_example::reqwest_go_1().await;
    reqwest_example::reqwest_example::reqwest_go_2().await;
    reqwest_example::reqwest_example::reqwest_go_3().await;
    reqwest_example::reqwest_example::reqwest_go_4().await;
}