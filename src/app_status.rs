#[derive(Debug)]
pub enum AppStatus {
    Empty,
    RetrieverCreated,
    PopulatingDB,
    ReadyForSearch,
    Searching,
}