#[derive(Clone)]
pub struct ProviderMetadata {
    pub name: String,
}

pub struct ResolutionError {
    pub code: String,
    pub message: String,
}
pub struct ResolutionDetails<T> {
    pub value: T,
    pub resolution_error: ResolutionError,
    pub reason: String,
    pub variant: String,
}
