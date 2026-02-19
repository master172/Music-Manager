pub trait AppInterface {
    fn create_playlist(&mut self, name: String);
    fn delete_playlist(&mut self, name: String);
    fn enter_playlist(&mut self, name: String);

    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn seek(&mut self, time: i32);
    fn repeat(&mut self, count: i32);
    fn play_selected(&mut self, path: String);

    fn search(&mut self, query: String, limit: usize);
    fn add(&mut self, link: String);

    fn return_to_main(&mut self);

    fn help(&mut self);

    fn list(&mut self);
    fn delete_song(&mut self, name: String);
    fn quit(&mut self);
}
