pub trait MetaConfig {
    fn init() -> Result<Self, std::error::Error>
    where
       Self: Sized;
}