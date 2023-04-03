pub mod setu;
pub mod test_1;
pub mod mc_status;


#[macro_export]
macro_rules! seg_to_vec {
    ($seg:expr) => {{
        let vec = $seg
        .iter()
        .filter_map(|msg| {
            return if msg.ty == "text" {
                match msg.data.to_owned().get("text") {
                    None => None,
                    Some(v) => match v {
                        walle::walle_core::prelude::Value::Str(str) => Some(str.clone()),
                        _ => return None,
                    },
                }
            } else {
                None
            };
        })
        .collect::<Vec<_>>();
        let vec = vec.get(0).unwrap_or(&"".to_string())
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        vec
    }};
}
#[macro_export]
macro_rules! string_to_i64 {
    ($string:expr) => {{
        $string.parse::<i64>().unwrap_or_default()
    }};
}
