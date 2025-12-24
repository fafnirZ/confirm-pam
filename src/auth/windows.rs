use super::BiometricAuthenticator;
use anyhow::Result;

pub struct WindowsAuthenticator;
use futures::executor;

impl WindowsAuthenticator {
    pub fn new() -> Result<Self> {
        Ok(WindowsAuthenticator)
    }
}

impl BiometricAuthenticator for WindowsAuthenticator {
    fn authenticate(&self, _message: &str) -> Result<bool> {
        // TODO: Implement Windows Hello authentication
        Err(anyhow::anyhow!(
            "Windows Hello authentication not yet implemented"
        ))
    }

    fn is_available(&self) -> Result<bool> {
        let future = is_windows_hello_available();
        executor::block_on(future)
    }
}
