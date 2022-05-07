#[derive(Debug)]
pub enum StatusUpdate<U, F> {
    Update(U),
    Finish(F),
}
