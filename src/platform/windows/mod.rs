// Windows platform implementation
use windows::{
    Security::Credentials::UI::{
        UserConsentVerifierAvailability,
        UserConsentVerifier,
        UserConsentVerificationResult,
    },
    Win32::UI::WindowsAndMessaging::{
        GetDesktopWindow
    },
    Win32::System::WinRT::IUserConsentVerifierInterop,
    core::{factory, HSTRING},
};
use windows_future::IAsyncOperation;
use anyhow::Result;

pub async fn is_windows_hello_available() -> Result<bool> {
    let availability = UserConsentVerifier::CheckAvailabilityAsync()?.await?;
    match availability {
        UserConsentVerifierAvailability::Available | UserConsentVerifierAvailability::DeviceBusy => Ok(true),
        _ => Ok(false)
    }
}

pub async fn authenticate_with_windows_hello(message: &str) -> Result<bool> {

    // using GetDesktopWindow for the hwnd auto focuses the prompt
    let hwnd = unsafe { GetDesktopWindow() };

    let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;
    let async_op: IAsyncOperation<UserConsentVerificationResult> = unsafe {
        interop.RequestVerificationForWindowAsync(hwnd, &HSTRING::from(message))?
    };
    let result = async_op.await?;
    match result {
        UserConsentVerificationResult::Verified => Ok(true),
        UserConsentVerificationResult::DeviceBusy | UserConsentVerificationResult::Canceled => Ok(false),
        UserConsentVerificationResult::RetriesExhausted => {
            Err(anyhow::anyhow!("Too many failed attempts"))
        },
        UserConsentVerificationResult::DeviceNotPresent 
            | UserConsentVerificationResult::DisabledByPolicy
            | UserConsentVerificationResult::NotConfiguredForUser => {
                Err(anyhow::anyhow!("Biometric authentication not available"))
            },
        _ => Err(anyhow::anyhow!("Unknown authentication error")),
    }
}