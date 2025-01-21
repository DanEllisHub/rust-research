pub mod missing_framing_policy {
    use std::ptr::null;

    use log::{info, warn, error, debug, trace, log};
    use base64::prelude::*;
    use rocket::shield::Shield;
    
    pub fn get_vulnerable_shield_0() -> Shield{
        // new empty shield
        let shield = Shield::new(); // Desanitizer - Missing Framing Policy
        return shield;
    }

    pub fn get_vulnerable_shield_1() -> Shield{
        // default shield, framing policy manually disabled
        let shield = Shield::default()
            .disable::<rocket::shield::Frame>(); // Desanitizer - Missing Framing Policy
        return shield;
    }
    pub fn get_safe_shield_2() -> Shield{
        // new empty shield, framing policy manually enabled
        let shield = Shield::new()
            .enable(rocket::shield::Frame::SameOrigin); // Sanitizer - Missing Framing Policy
        return shield;
    }
    pub fn get_safe_shield_3() -> Shield{
        // new empty shield, framing policy manually enabled
        let shield = Shield::new()
            .enable(rocket::shield::Frame::Deny); // Sanitizer - Missing Framing Policy 
            
        return shield;
    }

    pub fn get_safe_shield_4() -> Shield{
        // default shield
        let shield = Shield::default(); // Sanitizer - Missing Framing Policy 
        return shield;
    }


    // pub fn get_some_shield() -> Shield{
    //     let shield = Shield::new()
    //         .enable(rocket::shield::Frame::Deny) // Sanitizer - Missing Framing Policy 
    //         .enable(rocket::shield::Frame::SameOrigin) // Sanitizer - Missing Framing Policy
    //         .enable(rocket::shield::NoSniff::Enable) // Sanitizer - Content-Type Options
    //         .disable::<rocket::shield::NoSniff>() // Desanitizer - Content-Type Options
    //         .disable::<rocket::shield::Frame>() // Desanitizer - Missing Framing Policy
    //         .disable::<rocket::shield::Hsts>() // Desanitizer - HSTS
    //         .enable(rocket::shield::Referrer::NoReferrer)
    //         .enable(rocket::shield::Prefetch::Off);
    //     return shield;
    // }
}