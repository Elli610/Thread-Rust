pub struct ThreadPool;

enum Error {
    PoolCreationError,
}
&
impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, Error> {

        assert!(size > 0);

        Ok(ThreadPool)
    }

    pub fn build (size: usize) -> ThreadPool {
        if size == 0 {
            return PoolCreationError;
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        // ...
    }

}