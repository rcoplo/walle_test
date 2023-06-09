use crate::config::RcoBotConfig;
use rbatis::Rbatis;

pub mod implement;
mod mapper;
pub mod table;

pub fn init_rbatis(config: &RcoBotConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 botconfig.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
