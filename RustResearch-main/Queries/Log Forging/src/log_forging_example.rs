pub mod log_forging_example {
    use log::{info, warn, error, debug, trace, log};
    use base64::prelude::*;

    pub fn forge(input: &str) {
        info!("Input: {}", input);  // RESULT
        warn!("Input: {}", input);  // RESULT
        error!("Input: {}", input); // RESULT
        error!(target: "TARGET", "ERROR_TARGET: {}{}", input, input); // 2 RESULTS
        debug!("Input: {}", input); // RESULT
        trace!("Input: {}", input); // RESULT
        log!(log::Level::Info, "Input: {}", input); // RESULT
    }

    pub fn safe(input: &str) {
        let input = input.replace("\n", "").replace("\r", "").replace("\t", ""); // SANITIZED
        info!("Input: {}", input); // SAFE
        warn!("Input: {}", input); // SAFE
        error!("Input: {}", input); // SAFE
        error!(target: "MY_TARGET", "ERROR_WITH_TARGET: {}{}", input, input); // SAFE
        debug!("Input: {}", input); // SAFE
        trace!("Input: {}", input); // SAFE
        log!(log::Level::Info, "Input: {}", input); // SAFE
    }

    pub fn safe_2(input: &str) {
        let encoded = BASE64_STANDARD.encode(input.as_bytes()); // SANITIZED
        info!("Input: {}", encoded); // SAFE
        warn!("Input: {}", encoded); // SAFE
        error!("Input: {}", encoded); // SAFE
        debug!("Input: {}", encoded); // SAFE
        trace!("Input: {}", encoded); // SAFE
        log!(log::Level::Info, "Input: {}", encoded); // SAFE
    }
}
