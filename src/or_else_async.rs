#[allow(async_fn_in_trait)]
pub trait OrElseAsync<T> {
    async fn unwrap_or_else_async(self, f: impl AsyncFnOnce() -> T) -> T;
}

impl<T> OrElseAsync<T> for Option<T> {
    async fn unwrap_or_else_async(self, f: impl AsyncFnOnce() -> T) -> T {
        match self {
            Some(t) => t,
            None => f().await,
        }
    }
}

impl<T, E> OrElseAsync<T> for Result<T, E> {
    async fn unwrap_or_else_async(self, f: impl AsyncFnOnce() -> T) -> T {
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
        assert_eq!(0, a.unwrap_or_else_async(async || 10000).await);

        let b = None;
        assert_eq!(10000, b.unwrap_or_else_async(async || 10000).await);
    }

    #[tokio::test]
    async fn test_result() {
        let a: Result<i32, ()> = Ok(0);
        assert_eq!(0, a.unwrap_or_else_async(async || 10000).await);

        let b: Result<i32, ()> = Err(());
        assert_eq!(10000, b.unwrap_or_else_async(async || 10000).await);
    }
}
