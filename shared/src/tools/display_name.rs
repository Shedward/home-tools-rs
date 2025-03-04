pub trait DisplayingName {
    fn display_name(&self) -> String;
}

impl<Wrapped: DisplayingName> DisplayingName for Option<Wrapped> {
    fn display_name(&self) -> String {
        match self {
            Some(wrapped) => wrapped.display_name(),
            None => "None".to_string(),
        }
    }
}
