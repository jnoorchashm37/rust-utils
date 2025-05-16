#[allow(async_fn_in_trait)]
pub trait OrElseAsync<T> {
    async fn async_unwrap_or_else(self, f: impl AsyncFnOnce() -> T) -> T;
}

impl<T> OrElseAsync<T> for Option<T> {
    async fn async_unwrap_or_else(self, f: impl AsyncFnOnce() -> T) -> T {
        match self {
            Some(t) => t,
            None => f().await,
        }
    }
}

impl<T, E> OrElseAsync<T> for Result<T, E> {
    async fn async_unwrap_or_else(self, f: impl AsyncFnOnce() -> T) -> T {
        match self {
            Ok(t) => t,
            Err(_) => f().await,
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[tokio::test]
    async fn test_option() {
        let a = Some(0);
        assert_eq!(0, a.async_unwrap_or_else(async || 10000).await);

        let b = None;
        assert_eq!(10000, b.async_unwrap_or_else(async || 10000).await);
    }

    #[tokio::test]
    async fn test_result() {
        let a: Result<i32, ()> = Ok(0);
        assert_eq!(0, a.async_unwrap_or_else(async || 10000).await);

        let b: Result<i32, ()> = Err(());
        assert_eq!(10000, b.async_unwrap_or_else(async || 10000).await);
    }
}
