//this is going to be the only thing in this file for now, but I will likely add stuff
pub trait Indexable<T> {
    fn index<'a>(&'a self, index: usize) -> Result<&'a T, String>;

    fn index_mut<'a>(&'a mut self, index: usize) -> Result<&'a mut T, String>;
}
