use std::net::*;
use hickory_resolver::{config::*, lookup::ReverseLookup, proto::rr::RData, Resolver};

use dns_lookup::{lookup_addr};

//use trust_dns_resolver::Resolver;
//use trust_dns_resolver::config::*;

fn main() {
    // Define the IP address for reverse lookup
    let ip_address: IpAddr = "140.82.121.3".parse().expect("Invalid IP address");

    // Create a new resolver
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).expect("Failed to create resolver");

    // Perform the reverse lookup
    match resolver.reverse_lookup(ip_address) {
        Ok(hostnames) => {
            if let Some(hostname) = hostnames.iter().next() {
                println!("[Hickory_dns] Hostname: {}", hostname);
                // check if the hostname contains the word github
                if hostname.to_string().contains("github") {
                    println!("[Hickory_dns] Hostname contains github");
                } else {
                    println!("[Hickory_dns] Hostname does not contain github");
                }
            } else {
                println!("[Hickory_dns] No hostname found");
            }
        }
        Err(err) => {
            eprintln!("[Hickory_dns] Reverse lookup failed: {}", err);
        }
    }

    // dns-lookup crate
    // let ip: std::net::IpAddr = "142.250.185.4".parse().unwrap();
    let host = lookup_addr(&ip_address).unwrap();
    println!("[dns-lookup] Host: {:?}", host);

    // check if host contains the word github
    if host.contains("github") {
        println!("[dns-lookup] Hostname contains github");
    } else {
        println!("[dns-lookup] Hostname does not contain github");
    }

}

// Example Rocket route with hostname parameter
// #[get("/reverse_lookup/<hostname>")]
// fn reverse_lookup(hostname: String) -> String {
//     // Perform the reverse lookup
//     let ip_address: IpAddr = hostname.parse().expect("Invalid IP address");
//     let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).expect("Failed to create resolver");

//     match resolver.reverse_lookup(ip_address) {
//         Ok(hostnames) => {
//             if let Some(hostname) = hostnames.iter().next() {
//                 println!("[Hickory_dns] Hostname: {}", hostname);
//                 // Check if the hostname contains the word github
//                 if hostname.to_string().contains("github") {
//                     // Do something in the case that the hostname contains github
//                     println!("[Hickory_dns] Hostname contains github");
//                 } else {
//                     println!("[Hickory_dns] Hostname does not contain github");
//                 }
//                 format!("Hostname: {}", hostname)
//             } else {
//                 "No hostname found".to_string()
//             }
//         }
//         Err(err) => {
//             println!("Reverse lookup failed: {}", err);
//             format!("Reverse lookup failed: {}", err)
//         }
//     }
// }

// Same as above but using the dns-lookup crate
// #[get("/reverse_lookup/<hostname>")]
// fn reverse_lookup(hostname: String) -> String {
//     // Perform the reverse lookup
//     let ip_address: IpAddr = hostname.parse().expect("Invalid IP address");
//     let host = lookup_addr(&ip_address).unwrap();
//     println!("[dns-lookup] Host: {:?}", host);

//     // Check if host contains the word github
//     if host.contains("github") {
//         // Do something in the case that the hostname contains github
//         println!("[dns-lookup] Hostname contains github");
//     } else {
//         println!("[dns-lookup] Hostname does not contain github");
//     }

//     format!("Hostname: {}", host)
// }