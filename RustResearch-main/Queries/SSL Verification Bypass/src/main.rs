mod rustls_example;
mod openssl_example;
mod reqwest_example;
mod tokio_example;
use std::{io::{stdout, Read, Write}, sync::Arc};
use rustls::RootCertStore;
use std::process::Command;

#[tokio::main]
async fn main() {
    rustls_example::rustls_example::rustls_go();
    openssl_example::openssl_example::openssl_go();
    openssl_example::openssl_example::openssl_go_2();
    reqwest_example::reqwest_example::reqwest_go_1().await;
    reqwest_example::reqwest_example::reqwest_go_2().await;
}