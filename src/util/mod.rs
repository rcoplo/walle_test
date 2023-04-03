mod http_util;

use std::path::PathBuf;
use walle::walle_core::prelude::{MsgSegment, Segments, Value, ValueMap};
use walle::walle_core::segment::{Mention, Reply, Text};

pub use http_util::{http_get, http_get_image, http_post_json};

pub struct MessageChain {
    inner: Segments,
}

impl MessageChain {
    pub fn new() -> MessageChain {
        Self {
            inner: Segments::new(),
        }
    }

    pub fn text(&mut self, text: &str) -> &mut MessageChain {
        self.inner.push(MsgSegment::from(Text {
            text: text.to_string(),
        }));
        self
    }

    pub fn reply(&mut self, message_id: &str, user_id: &str) -> &mut MessageChain {
        self.inner.push(MsgSegment::from(Reply {
            message_id: message_id.to_string(),
            user_id: user_id.to_string(),
        }));
        self
    }

    pub fn at(&mut self, user_id: &str) -> &mut MessageChain {
        self.inner.push(MsgSegment::from(Mention {
            user_id: user_id.to_string(),
        }));
        self
    }

    pub fn image_bytes(&mut self, bytes: Vec<u8>) -> &mut MessageChain {
        let mut map = ValueMap::new();
        map.insert("file_id".to_string(), Value::from(""));
        map.insert("bytes".to_string(), Value::from(bytes));
        self.inner.push(MsgSegment {
            ty: "image".to_string(),
            data: map,
        });
        self
    }

    pub fn image(&mut self, image: &str) -> &mut MessageChain {
        let mut map = ValueMap::new();
        map.insert("file_id".to_string(), Value::from(""));
        map.insert("url".to_string(), Value::from(image));
        self.inner.push(MsgSegment {
            ty: "image".to_string(),
            data: map,
        });
        self
    }

    pub fn build(&self) -> Segments {
        self.inner.clone()
    }
}

pub struct Resources;

impl Resources {
    fn new() {
        // let mut config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // config_path.push("image");
        // config_path.to_str()
    }
}
#[macro_export]
macro_rules! resource_path {
    ($($path:literal),* => $file_name:literal) => {{
         resource_path!($($path),* => Some($file_name))
    }};

    ($($path:literal),*) => {{
        resource_path!($($path),* => None::<String>)
    }};

    ($($path:literal),* => $file_name:expr) => {{
         let mut config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
         config_path.push("resources");
         $(config_path.push($path);)*
         if let Some(file_name) =  $file_name {
             config_path.push(file_name);
         }
         config_path.to_str().map(|v| v.to_string())
     }}
}

pub trait ToNum {
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
}

impl ToNum for String {
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().unwrap_or_default()
    }
    fn to_i64(&self) -> i64 {
        self.parse::<i64>().unwrap_or_default()
    }
}

#[test]
fn test() {
    let image = resource_path!("image","hao" => "");
    println!("{:?}", image);
    assert_eq!(
        image,
        Some(r"D:\data\my\rcoplo\rust\walle_test\resources\image\hao\".to_string())
    )
}
