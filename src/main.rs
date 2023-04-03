use walle::MatcherHandler;
use walle_test::CONTEXT;
use walle_test::plugins::{
    mc_status, setu
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CONTEXT.init_pool().await;
    let mut matchers_config = walle::MatchersConfig::default();
    matchers_config.nicknames = CONTEXT.config.bot_config.bot_name.clone();
    let matchers = walle::Matchers::default()
        .add_matcher(setu::setu().boxed())
        .add_matcher(mc_status::mc_status().boxed());
    let walle = walle::new_walle(matchers, "debug");
    walle
        .start(
            walle::config::AppConfig::default(),
            matchers_config,
            true,
        )
        .await
        .unwrap();
    walle.wait_all().await;
    Ok(())
}
