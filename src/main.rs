//#![windows_subsystem = "windows"]
use std::time::Duration;
use slint::{CloseRequestResponse, SharedString};
slint::include_modules!();
use env_logger::Env;

mod camera;
mod utils;
mod model;
use camera::open_camera;
use utils::open_folder;

use log::{info,debug};

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    debug!("日志初始化成功");
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    let win = HelloWorld::new().unwrap();
    

    win.on_execute({
        let win_weak = win.as_weak();
        move || {
            let win_weak = win_weak.clone();
            tokio::spawn(async move {
                for i in 0..11{
                    std::thread::sleep(Duration::from_millis(1000));
                    let _ = win_weak.upgrade_in_event_loop(move |hello|{
                        hello.set_name(format!("{}",11 - i).into());
                    });
                }
            });
        }
    });

    win.on_clicked(|_e:i32|{
        open_camera();
    });
    
    let compress = win.as_weak().unwrap();
    win.on_compress_open(move ||{
        info!("select folder");
        let path = open_folder();
        if !path.eq("") {
            //输出目录到页面
            compress.global::<CompressGlobal>().set_compress_path(SharedString::from("123"));
        }
    });

    //监听关闭窗体按钮
    win.window().on_close_requested(|| {
        //关闭
        CloseRequestResponse::HideWindow
        //不可以关闭
        //CloseRequestResponse::KeepWindowShown
    });
    
    info!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    
    let _ = win.run();
    info!("Program exit!!!");
}
