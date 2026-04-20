use deboa::{
    errors::{DeboaError, IoError},
    response::DeboaResponse,
    Result,
};
use futures::StreamExt;
use std::{fs::File, io::Write, path::Path};

/// Helper type to write a DeboaResponse to a file.
pub struct ToFile {
    response: DeboaResponse,
}

/// Trait to convert a DeboaResponse into a file writer.
///
/// This trait allows converting a DeboaResponse into a ToFile helper type
/// which provides async file writing capabilities.
///
/// # Example
/// ``` rust, compile_fail
/// use deboa::{request::get, Client};
/// use deboa_extras::http::utils::file::IntoFile;
///
/// let mut client = Client::new();
/// let response = get("https://example.com")
///     .send_with(&mut client)
///     .await
///     .unwrap();
/// response
///     .into_file()
///     .save("output.txt", None)
///     .await
///     .unwrap();
/// ```
pub trait IntoFile {
    /// Convert the response into a file writer
    fn into_file(self) -> ToFile;
}

impl IntoFile for DeboaResponse {
    fn into_file(self) -> ToFile {
        ToFile { response: self }
    }
}

impl ToFile {
    ///
    /// Save the response body to a file asynchronously.
    ///
    /// # Arguments
    /// * `path` - The path where the file will be saved
    /// * `on_progress` - Optional callback function that receives the number of bytes written
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Err with IoError if failed
    ///
    /// # Examples
    /// ``` rust, compile_fail
    ///
    /// use deboa::{request::get, Deboa};
    /// use deboa_extras::http::utils::file::IntoFile;
    ///
    /// let mut client = Deboa::default();
    /// let response = get("https://example.com").send_with(&mut client).await?;
    /// response.into_file().save("output.txt", None).await?;
    /// ```
    pub async fn save<P, EV>(self, path: P, on_progress: Option<EV>) -> Result<()>
    where
        P: AsRef<Path> + Send,
        EV: Fn(u64) + Send + Sync + 'static,
    {
        let file = File::create(path.as_ref());
        if let Err(err) = file {
            return Err(DeboaError::Io(IoError::File { message: err.to_string() }));
        }

        let mut file = file.unwrap();
        let mut stream = self
            .response
            .stream();
        while let Some(frame) = stream.next().await {
            if let Ok(chunk) = frame {
                if let Some(data) = chunk.data_ref() {
                    if let Some(ref on_progress) = on_progress {
                        on_progress(data.len() as u64);
                    }
                    if let Err(err) = file.write(data) {
                        return Err(DeboaError::Io(IoError::File { message: err.to_string() }));
                    }
                }
            }
        }

        if let Err(err) = file.flush() {
            return Err(DeboaError::Io(IoError::File { message: err.to_string() }));
        }

        Ok(())
    }
}
