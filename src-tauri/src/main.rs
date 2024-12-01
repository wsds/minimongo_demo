// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::run_mini_mongo::start_mini_mongo_server_sub_thread;

mod run_mini_mongo;

fn main() {
    start_mini_mongo_server_sub_thread();
    minimongo_demo_lib::run()
}
