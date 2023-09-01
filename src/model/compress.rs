

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ImageList{
    pub name:slint::SharedString,
    pub path:slint::SharedString,
    pub status:i32,
    pub format:slint::SharedString
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ImageListData{
    pub name:String,
    pub path:String,
    pub status:i32,
    pub format:String
}
