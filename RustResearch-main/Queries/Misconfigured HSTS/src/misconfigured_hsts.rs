pub mod misconfigured_hsts {
    use std::ptr::null;

    use log::{info, warn, error, debug, trace, log};
    use base64::prelude::*;
    use rocket::shield::Shield;
    
    pub fn get_vulnerable_shield_0() -> Shield{
        // default shield
        let shield = Shield::default(); // HSTS is improperly defined
        return shield;
    }
    pub fn get_vulnerable_shield_1() -> Shield{
        // default shield
        let shield = Shield::default()
            .enable(rocket::shield::Hsts::Enable(rocket::time::Duration::days(365))); // HSTS is improperly defined
        return shield;
    }

    pub fn get_vulnerable_shield_2() -> Shield{
        // default shield
        let shield = Shield::new()
            .enable(rocket::shield::Hsts::Enable(rocket::time::Duration::days(365))); // HSTS is improperly defined
        return shield;
    }

    pub fn get_safe_shield_2() -> Shield{
        // new empty shield, HSTS manually enabled with relevant directives
        let shield = Shield::new()
            .enable(rocket::shield::Hsts::IncludeSubDomains(rocket::time::Duration::days(365))); // Sanitizer - misconfigured HSTS
        return shield;
    }

    pub fn get_safe_shield_3() -> Shield{
        // new empty shield, HSTS manually enabled
        let shield = Shield::default()
            .enable(rocket::shield::Hsts::IncludeSubDomains(rocket::time::Duration::days(365))); // Sanitizer - misconfigured HSTS
        return shield;
    }


    // pub fn get_some_shield() -> Shield{
    //     let shield = Shield::new()
    //         .enable(rocket::shield::Frame::Deny) // Sanitizer - misconfigured HSTS 
    //         .enable(rocket::shield::Frame::SameOrigin) // Sanitizer - misconfigured HSTS
    //         .enable(rocket::shield::NoSniff::Enable) // Sanitizer - Content-Type Options
    //         .disable::<rocket::shield::NoSniff>() // Desanitizer - Content-Type Options
    //         .disable::<rocket::shield::Frame>() // Desanitizer - misconfigured HSTS
    //         .disable::<rocket::shield::Hsts>() // Desanitizer - HSTS
    //         .enable(rocket::shield::Referrer::NoReferrer)
    //         .enable(rocket::shield::Prefetch::Off);
    //     return shield;
    // }
}