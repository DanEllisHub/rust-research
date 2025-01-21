use std::{io::{stdout, Read, Write}, sync::Arc};
use rustls::RootCertStore;

pub mod rustls_example {
    use std::io::{stdout, Read, Write};
    use std::sync::Arc;

    use rustls::client::danger::{HandshakeSignatureValid};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use rustls::{DigitallySignedStruct, RootCertStore};

    #[derive(Debug)]
    pub struct NoCertificateVerification {} 
    impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp: &[u8],
            _now: UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            // unsafe veirification:
            
            Ok(rustls::client::danger::ServerCertVerified::assertion()) // RESULT
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            Ok(HandshakeSignatureValid::assertion())  // RESULT
        } 

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            Ok(HandshakeSignatureValid::assertion())  // RESULT
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            let k : Vec<rustls::SignatureScheme> = vec![rustls::SignatureScheme::RSA_PKCS1_SHA1];
            k
        }
    }

    pub fn rustls_go() {
        // let root_store = RootCertStore::from_iter(
        //     webpki_roots::TLS_SERVER_ROOTS
        //         .iter()
        //         .cloned(),
        // );
        let verifier = Arc::new(NoCertificateVerification {});
        let mut config = rustls::ClientConfig::builder()
            .with_root_certificates(RootCertStore::empty())
            .with_no_client_auth();
        config.dangerous().set_certificate_verifier(verifier.clone());
        // convert the string to ServerName
        let server_name = ServerName::try_from("expired.badssl.com").unwrap();
        let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
        let mut sock = std::net::TcpStream::connect("expired.badssl.com:443").unwrap();
        let mut tls = rustls::Stream::new(&mut conn, &mut sock);
        tls.write_all(
            concat!(
                "GET / HTTP/1.1\r\n",
                "Host: expired.badssl.com\r\n",
                "Connection: close\r\n",
                "Accept-Encoding: identity\r\n",
                "\r\n"
            )
            .as_bytes(),
        )
        .unwrap();
        let ciphersuite = tls
            .conn
            .negotiated_cipher_suite()
            .unwrap();
        writeln!(
            &mut std::io::stderr(),
            "Current ciphersuite: {:?}",
            ciphersuite.suite()
        )
        .unwrap();
        let mut plaintext = Vec::new();
        tls.read_to_end(&mut plaintext).unwrap();
        stdout().write_all(&plaintext).unwrap();
    }
}

