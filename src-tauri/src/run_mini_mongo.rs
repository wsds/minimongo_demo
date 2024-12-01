use std::thread;

pub fn start_mini_mongo_server_sub_thread(){
    thread::spawn(|| {
        println!("start mini_mongo_server in sub_thread. 1002");
        let _ = minimongo::start_mmg_server();
        println!("start mini_mongo_server finished. 1002");
    });
}