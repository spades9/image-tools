use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::core::Vector;
use opencv::prelude::*;
use opencv::imgcodecs::imwrite;


pub fn open_camera(){
    let mut cap = VideoCapture::default().unwrap();
    let mut mat = Mat::default();
    let _ = cap.release();
    let res = cap.open_file("E:\\rust\\test.mp4", CAP_ANY);
    println!("{:?}",res);
    println!("{:?}",cap.is_opened());
    for i in 0..10 {
        let _ = cap.read(&mut mat);
        println!("{:?}",&mat.size());
        //let v = Vector::default();
        //println!("保存图片");
        //let name = format!("E:/rust/{}.png",i);
        //let _ = imwrite(&name, &mat, &v);
    }
    
}


