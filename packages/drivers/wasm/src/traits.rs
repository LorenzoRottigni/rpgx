pub trait WasmWrapper<T> {
    /// Get a reference to the inner type (Rust struct)
    fn inner(&self) -> &T;

    /// Consume the wrapper and return the inner type
    fn into_inner(self) -> T;

    /// Create a new wrapper from the inner type
    fn from_inner(inner: T) -> Self;
}
