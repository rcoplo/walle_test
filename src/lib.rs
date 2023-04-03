use crate::config::RcoBotConfig;
use crate::database::implement::mc_server_impl::McServerImpl;
use crate::database::table::*;
use once_cell::sync::Lazy;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;

mod api;
mod config;
mod database;
mod error;
pub mod plugins;
pub mod util;

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub static CONTEXT: Lazy<BotConText> = Lazy::new(|| BotConText::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::CONTEXT.rbatis.clone()
    };
}

pub struct BotConText {
    pub config: RcoBotConfig,
    pub rbatis: Rbatis,
    pub mc_server: McServerImpl,
}

impl Default for BotConText {
    fn default() -> Self {
        let config = RcoBotConfig::default();
        Self {
            rbatis: database::init_rbatis(&config),
            config,
            mc_server: McServerImpl {},
        }
    }
}

impl BotConText {
    pub async fn init_pool(&self) {
        let path = resource_path!("data" => "bot.db").unwrap_or_default();
        walle::tracing::debug!("database path: {}", &path);
        self.rbatis.init(SqliteDriver {}, &path).unwrap();
        let mut s = SqliteTableSync::default();
        s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
        // bili_push
        s.sync(self.rbatis.acquire().await.unwrap(), to_value!(BiliPush {
            id: Some(0),
            ..Default::default()
        }), "bili_push")
            .await
            .unwrap();
        // osu_sb
        s.sync(
            self.rbatis.acquire().await.unwrap(),
            to_value!(OsuSb {
                id: Some(0),
                ..Default::default()
            }),
            "osu_sb",
        )
        .await
        .unwrap();
        // Sign
        s.sync(
            self.rbatis.acquire().await.unwrap(),
            to_value!(Sign {
                id: Some(0),
                ..Default::default()
            }),
            "sign",
        )
        .await
        .unwrap();
        // EttUser
        s.sync(
            self.rbatis.acquire().await.unwrap(),
            to_value!(EttUser {
                id: Some(0),
                ..Default::default()
            }),
            "ett_user",
        )
        .await
        .unwrap();
        // McStatusData
        s.sync(
            self.rbatis.acquire().await.unwrap(),
            to_value!(McServer {
                id: Some(0),
                ..Default::default()
            }),
            "mc_server",
        )
        .await
        .unwrap();
    }
}

