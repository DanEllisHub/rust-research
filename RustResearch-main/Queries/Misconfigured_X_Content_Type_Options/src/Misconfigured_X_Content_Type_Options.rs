pub mod Misconfigured_X_Content_Type_Options {
    use std::ptr::null;

    use log::{info, warn, error, debug, trace, log};
    use base64::prelude::*;
    use rocket::shield::Shield;
    
    pub fn get_vulnerable_shield_0() -> Shield{
        // default shield
        let shield = Shield::default()
            .disable::<rocket::shield::NoSniff>(); // HSTS is improperly defined
        return shield;
    }
    pub fn get_vulnerable_shield_1() -> Shield{
        // default shield
        let shield = Shield::new();
        return shield;
    }

    pub fn get_safe_shield_2() -> Shield{
        // new empty shield, HSTS manually enabled with relevant directives
        let shield = Shield::new()
            .enable(rocket::shield::NoSniff::Enable); // Sanitizer - misconfigured HSTS
        return shield;
    }

    pub fn get_safe_shield_3() -> Shield{
        // new empty shield, HSTS manually enabled
        let shield = Shield::default();
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