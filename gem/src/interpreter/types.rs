//this is going to be the only thing in this file for now, but I will likely add stuff
pub trait Indexable<T> {
    fn index<'a>(&'a self, index: usize) -> Result<&'a T, String>;
}
