#[macro_export]
macro_rules! String{
    ($str:expr) => {
        $str.to_string()
    }
}



pub fn file_name(name:&str,extension:&str) -> String
{
    let extens = ".".to_owned() + extension;
    for i in 0..name.len()
    {
        if name[i..] == extens {
            return String!(name[..i]);
        }
    }
    return String!("");
}