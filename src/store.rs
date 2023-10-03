use redis::AsyncCommands;


pub trait Store {
    fn put();
    fn get();
    fn get_all();
    fn delete();
}
