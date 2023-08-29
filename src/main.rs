//#![windows_subsystem = "windows"]
use std::{time::Duration, rc::Rc};
use slint::{CloseRequestResponse, SharedString, ModelRc, VecModel};
slint::include_modules!();
use env_logger::Env;
use std::fs::{read_dir,metadata};
mod utils;
mod model;
use utils::open_folder;

use log::{info,debug};

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    debug!("日志初始化成功");
    
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
        
    });
    
    let compress = win.as_weak().unwrap();
    win.on_compress_open(move ||{
        info!("select folder");
        let path = open_folder();
        if !path.eq("") {
            //输出目录到页面
            compress.global::<CompressGlobal>().set_compress_path(SharedString::from(&path));
            let image_list = compress.global::<CompressGlobal>().get_image_list();
            let mut new_image_list = Vec::new();
            println!("{:?}",image_list);
            //获取到这个目录之后获取这个文件夹下的所有图片文件
            let read_result = read_dir(&path);
            if let Ok(dirs) = read_result {
                for entiy in dirs {
                    if let Ok(dir_entry) = entiy {
                        let path = dir_entry.path();
                        if let Ok(f) = metadata(&path){
                            if f.is_file() {
                                if let Some(file_name) = path.file_name(){
                                    info!("file_name:{}",file_name.to_str().unwrap());
                                    new_image_list.push(ImageList{
                                        name:SharedString::from(file_name.to_str().unwrap()),
                                        path:SharedString::from(file_name.to_str().unwrap()),
                                        status:0
                                    });
                                }
                            }
                        }
                    }
                }
            }
            compress.global::<CompressGlobal>().set_image_list(ModelRc::from(Rc::new(VecModel::from(new_image_list.clone()))));
        }
    });

    //监听关闭窗体按钮
    win.window().on_close_requested(|| {
        //关闭
        CloseRequestResponse::HideWindow
        //不可以关闭
        //CloseRequestResponse::KeepWindowShown
    });
    
    let _ = win.run();
    info!("Program exit!!!");
}
