#[derive(Debug, PartialEq)]
pub enum Loading<T, E> {
    Loading,
    Loaded(T),
    Failed(E),
}

impl<T, E> Loading<T, E>
where
    E: Clone,
{
    pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Loading<U, E> {
        match self {
            Self::Loading => Loading::Loading,
            Self::Loaded(val) => Loading::Loaded(f(val)),
            Self::Failed(err) => Loading::Failed(err.clone()),
        }
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
