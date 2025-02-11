use std::error::Error;

#[derive(Debug)]
pub enum Loading<T, E: Error> {
    Loading,
    Loaded(T),
    Failed(E),
}
