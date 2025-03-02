#[derive(Debug, PartialEq)]
pub enum Loading<T, E> {
    None,
    Loading,
    Loaded(T),
    Failed(E),
}

impl<T, E> Loading<T, E>
where
    E: Clone,
{
    pub fn from_result(result: Result<T, E>) -> Self {
        match result {
            Ok(val) => Loading::Loaded(val),
            Err(err) => Loading::Failed(err),
        }
    }

    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Loading<U, E> {
        match self {
            Self::None => Loading::None,
            Self::Loading => Loading::Loading,
            Self::Loaded(val) => Loading::Loaded(f(val)),
            Self::Failed(err) => Loading::Failed(err.clone()),
        }
    }
}

impl<T, E> Default for Loading<T, E> {
    fn default() -> Self {
        Loading::None
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;

    #[test]
    fn test() {
        let original = Loading::Loaded(1) as Loading<i32, Infallible>;
        let new = original.map(|v| v * 2);
        assert_eq!(new, Loading::Loaded(2));
    }
}
