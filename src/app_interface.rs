pub trait AppInterface {
    fn create_playlist(&mut self, name: String);
    fn delete_playlist(&mut self, name: String);
    fn enter_playlist(&mut self, name: String);

    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn seek(&mut self, time: i32);

    fn search(&mut self, query: String, limit: usize);
    fn add(&mut self, link: String);

    fn return_to_main(&mut self);

    fn help(&mut self);

    fn quit(&mut self);
}
