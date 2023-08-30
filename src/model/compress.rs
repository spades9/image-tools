

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ImageList{
    pub name:slint::SharedString,
    pub path:slint::SharedString,
    pub status:i32,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ImageListData{
    pub name:String,
    pub path:String,
    pub status:i32
}
