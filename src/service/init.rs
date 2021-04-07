use crate::service::id_generator::snow_worker::SnowWorkerM1;
use crate::service::id_generator::id_generator_options::IdGeneratorOptions;
use std::sync::{Mutex};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WORKPOLL:Mutex<Vec<SnowWorkerM1>> = Mutex::new(Vec::new());
}

pub fn run() {
    //let mut work_pool: Mutex<Vec<SnowWorkerM1>> = Mutex::new(Vec::new()) ;
    for i in 1..64 {
        //定义work_id 对应的配置
        let id_generator_options = IdGeneratorOptions::new(i as u32);
        //初始化对象
        let object = SnowWorkerM1::new(id_generator_options);
        //添加到数据组中
        WORKPOLL.lock().unwrap().push(object);
        println!("work_id {} 加载中", i)
    }
    println!("work_id_object加载完毕");
    //return work_pool;
}