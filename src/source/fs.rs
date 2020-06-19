use {
    crate::source::{Source, SourceError},
    maybe_sync::{BoxFuture, Rc},
    std::path::{Path, PathBuf},
};

/// Asset source that treats asset key as relative file path,
/// joins it with root path and loads asset data from file.
#[cfg_attr(all(doc, feature = "unstable-doc"), doc(cfg(feature = "fs")))]
#[derive(Debug)]
pub struct FileSource {
    root: PathBuf,
}

impl FileSource {
    /// Create new source with specified root path
    pub fn new(root: PathBuf) -> Self {
        FileSource { root }
    }
}

impl<P> Source<P> for FileSource
where
    P: AsRef<Path> + ?Sized,
{
    fn read(&self, path: &P) -> BoxFuture<'_, Result<Vec<u8>, SourceError>> {
        let path = self.root.join(path.as_ref());
        log::trace!("Fetching asset file at {}", &*path.to_string_lossy());
        let result = match std::fs::read(path) {
            Ok(bytes) => Ok(bytes),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Err(SourceError::NotFound),
            Err(err) => Err(SourceError::Error(Rc::new(err))),
        };

        Box::pin(async move { result })
    }
}
