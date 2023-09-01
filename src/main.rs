//#![windows_subsystem = "windows"]
use std::{time::Duration, rc::Rc, thread};
use slint::{CloseRequestResponse, SharedString, ModelRc, VecModel, Model};
slint::include_modules!();
use env_logger::Env;
use std::fs::{read_dir,metadata};
mod utils;
mod model;
use utils::{open_folder,get_image_format,convert_image_format};
use model::ImageListData;
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
    
    let compress = win.as_weak().unwrap();
    //Get directory
    win.global::<CompressGlobal>().on_open_folder(move ||{
        info!("select folder");
        let path = open_folder();
        if !path.eq("") {
            //Output to page
            compress.global::<CompressGlobal>().set_compress_path(SharedString::from(&path));
            let mut new_image_list = Vec::new();
            //Get all files under this folder
            let read_result = read_dir(&path);
            if let Ok(dirs) = read_result {
                for entiy in dirs {
                    if let Ok(dir_entry) = entiy {
                        let path = dir_entry.path();
                        if let Ok(f) = metadata(&path){
                            if f.is_file() {
                                if let Some(file_name) = path.file_name(){
                                    if let Some(format) = get_image_format(path.as_os_str().to_str().unwrap()){
                                        info!("format:{:?}",format);
                                        new_image_list.push(ImageList{
                                            name:SharedString::from(file_name.to_str().unwrap()),
                                            path:SharedString::from(path.to_str().unwrap()),
                                            status:0,
                                            format:SharedString::from(format)
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            compress.global::<CompressGlobal>().set_image_list(ModelRc::from(Rc::new(VecModel::from(new_image_list.clone()))));
        }
    });

    let system_global = win.as_weak().unwrap();
    win.global::<SystemGlobal>().on_system_close(move ||{
        let _ = system_global.hide();
    });

    let compress = win.as_weak();
    win.global::<CompressGlobal>().on_compress_batching({
        let win_compress = compress.clone();
        move || {
            let win_weak = compress.clone();
            //let mut count:i32 = 0;
            let image_list = win_compress.unwrap().global::<CompressGlobal>().get_image_list();
            let count = image_list.row_count();
            let mut list = Vec::new();
            for i in 0..count {
                if let Some(data) = image_list.row_data(i as usize){
                    list.push(
                        ImageListData{
                            name:data.name.to_string(),
                            path:data.path.to_string(),
                            status:data.status,
                            format:data.format.to_string()
                        }
                    );
                }
            }
            tokio::spawn(async move {
                info!("1-count:{}",count);
                for li in 0..list.len(){
                    info!("开始处理");
                    let _ = win_weak.clone().upgrade_in_event_loop(move |hello|{
                        info!("开始更新页面");
                        let image_list = hello.global::<CompressGlobal>().get_image_list();
                        let mut d = image_list.row_data(li as usize).unwrap();
                        d.status = 1;
                        info!("改变状态");
                        image_list.set_row_data(li as usize, d);
                        hello.window().request_redraw();
                    });
                    let data_image = list.get(li).unwrap();
                    info!("path:{}",data_image.path);
                    convert_image_format(&data_image.path, li as i32);
                    info!("处理完成");
                
                    let _ = win_weak.clone().upgrade_in_event_loop(move |hello|{
                        info!("开始更新页面");
                        let image_list = hello.global::<CompressGlobal>().get_image_list();
                        let mut d = image_list.row_data(li as usize).unwrap();
                        d.status = 2;
                        info!("改变状态");
                        image_list.set_row_data(li as usize, d);
                        hello.window().request_redraw();
                    });
                }
            });
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
