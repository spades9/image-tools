use std::time::Duration;
use slint::CloseRequestResponse;
slint::include_modules!();

mod camera;
use camera::open_camera;

#[tokio::main]
async fn main() {
    let win = HelloWorld::new().unwrap();
    
    win.on_execute({
        let win_weak = win.as_weak();
        move || {
            let win_weak = win_weak.clone();
            println!("开始执行");
            tokio::spawn(async move {
                for i in 0..11{
                    std::thread::sleep(Duration::from_millis(1000));
                    println!("执行结束{}",i);
                    let _ = win_weak.upgrade_in_event_loop(move |hello|{
                        hello.set_name(format!("{}",11 - i).into());
                    });
                }
            });
        }
    });

    win.on_clicked(|e:i32|{
        println!("e:{}",e);
        open_camera();
    });

    //监听关闭窗体按钮
    win.window().on_close_requested(|| {
        //关闭
        CloseRequestResponse::HideWindow
        //不可以关闭
        //CloseRequestResponse::KeepWindowShown
    });
    
    
    let _ = win.run();
    println!("Hello, world!");
}
