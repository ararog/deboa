use deboa::{response::DeboaResponse, Result};
use http_body_util::BodyExt;
use std::{io::Write, path::Path};


/// Trait to write a stream to a file
/// Used to download large files from a server
#[deboa::async_trait]
pub trait ToFile {
    /// Write a stream to a file
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to the file to write to
    /// * `on_progress` - A callback to be called when a chunk of the stream is written
    /// 
    /// # Returns
    /// 
    /// * `Result<()>` - Ok(()) if the stream was written successfully, Err(e) otherwise
    /// 
    /// # Examples
    /// 
    /// ```compile_fail
    /// let response = client.get("https://example.com/file").await?;
    /// response.to_file("file.txt", None).await?;
    /// ```
    async fn to_file<P, EV>(&mut self, _path: P, _on_progress: Option<EV>) -> Result<()>
    where
        P: AsRef<Path> + Send,
        EV: Fn(u64) + Send + Sync + 'static,
    {
        unimplemented!()
    }
}

#[deboa::async_trait]
impl ToFile for DeboaResponse {
    async fn to_file<P, EV>(&mut self, path: P, on_progress: Option<EV>) -> Result<()>
    where
        P: AsRef<Path> + Send,
        EV: Fn(u64) + Send + Sync + 'static,
    {
        let file = std::fs::File::create(path.as_ref());
        if let Err(err) = file {
            return Err(deboa::errors::DeboaError::Io { message: err.to_string() });
        }

        let mut file = file.unwrap();
        let mut stream = self.stream();
        while let Some(frame) = stream.frame().await {
            let frame = frame.unwrap();
            if let Some(chunk) = frame.data_ref() {
                if let Some(ref on_progress) = on_progress {
                    on_progress(chunk.len() as u64);
                }
                if let Err(err) = file.write(chunk) {
                    return Err(deboa::errors::DeboaError::Io { message: err.to_string() });
                }
            }
        }
        
        if let Err(err) = file.flush() {
            return Err(deboa::errors::DeboaError::Io { message: err.to_string() });
        }

        Ok(())
    }
}
