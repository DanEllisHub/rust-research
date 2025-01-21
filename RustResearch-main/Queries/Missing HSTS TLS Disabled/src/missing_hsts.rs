pub mod missing_hsts {
    use std::ptr::null;

    use log::{info, warn, error, debug, trace, log};
    use base64::prelude::*;
    use rocket::shield::Shield;
    
    pub fn get_vulnerable_shield_0() -> Shield{
        // new empty shield
        let shield = Shield::new(); // Desanitizer - Missing HSTS
        return shield;
    }

    pub fn get_vulnerable_shield_1() -> Shield{
        // default shield, HSTS manually disabled
        let shield = Shield::default()
            .disable::<rocket::shield::Hsts>(); // Desanitizer - Missing HSTS
        return shield;
    }

    pub fn get_vulnerable_shield_2() -> Shield{
        // default shield, tls feature is disabled
        let shield = Shield::default(); // Desanitizer - Missing HSTS 
        return shield;
    } 
    
    pub fn get_safe_shield_3() -> Shield{
        // new empty shield, HSTS manually enabled
        let shield = Shield::new()
            .enable(rocket::shield::Hsts::IncludeSubDomains(rocket::time::Duration::days(365))); // Sanitizer - Missing HSTS
        return shield;
    }

    pub fn get_safe_shield_4() -> Shield{
        // new empty shield, HSTS manually enabled
        let shield = Shield::new()
            .enable(rocket::shield::Hsts::Enable(rocket::time::Duration::days(365))); // Sanitizer - Missing HSTS
        return shield;
    }


    // pub fn get_some_shield() -> Shield{
    //     let shield = Shield::new()
    //         .enable(rocket::shield::Frame::Deny) // Sanitizer - Missing HSTS 
    //         .enable(rocket::shield::Frame::SameOrigin) // Sanitizer - Missing HSTS
    //         .enable(rocket::shield::NoSniff::Enable) // Sanitizer - Content-Type Options
    //         .disable::<rocket::shield::NoSniff>() // Desanitizer - Content-Type Options
    //         .disable::<rocket::shield::Frame>() // Desanitizer - Missing HSTS
    //         .disable::<rocket::shield::Hsts>() // Desanitizer - HSTS
    //         .enable(rocket::shield::Referrer::NoReferrer)
    //         .enable(rocket::shield::Prefetch::Off);
    //     return shield;
    // }
}