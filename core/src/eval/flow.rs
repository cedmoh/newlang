#[derive(Debug, Clone)]
pub enum Flow {
    Finished,
    Returned,
    Broke,
    Continued,
}
