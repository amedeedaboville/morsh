pub trait Transport: Send {
    fn test(&self) -> usize;
}
