/// basic meta action for all models
pub trait MetaAction {
    // string to struct?
    fn add<T>(t: T);
    // remove struct by id ?
    fn delete_by_id(id: i32);
    // file name id?
    fn update_by_id<T>(id: i32, t: T);

    fn config();
}
