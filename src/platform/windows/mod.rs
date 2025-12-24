// Windows platform implementation
use windows::Security::Credentials::UI::{
    UserConsentVerifierAvailability
}
use anyhow::Result;

pub async fn is_windows_hello_available() -> Result<bool> {
    let availability = UserConsentVerifier::CheckAvailabilityAsync()?.await?;
    match availability {
        UserConsentVerifierAvailability::Available | UserConsentVerifierAvailability::DeviceBusy => Ok(true)
        _ => Ok(false)
    }
}