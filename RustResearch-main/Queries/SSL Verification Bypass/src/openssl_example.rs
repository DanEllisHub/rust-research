pub mod openssl_example {
    use openssl::ssl::{SslConnector, SslMethod};
    use rocket::http::hyper::server::conn;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::ptr::null;
    
    pub fn openssl_go() {
        let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
        connector.set_verify(openssl::ssl::SslVerifyMode::NONE); // RESULT

        let s = connector.build();
        let stream = TcpStream::connect("expired.badssl.com:443").unwrap();
        let mut stream = s.connect("expired.badssl.com", stream).unwrap();
        
        stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        println!("{}", String::from_utf8_lossy(&res));
    }

    pub fn openssl_go_2() {
        let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
        //connector.set_verify(openssl::ssl::SslVerifyMode::NONE); // RESULT

        // write a callback that does nothing
        connector.set_verify_callback(openssl::ssl::SslVerifyMode::NONE, |_, _| false); // RESULT

        let s = connector.build();
        let stream = TcpStream::connect("expired.badssl.com:443").unwrap();
        let mut stream = s.connect("expired.badssl.com", stream).unwrap();
        
        stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        println!("{}", String::from_utf8_lossy(&res));
    }
}
