use crate::api::setu_api::LoliconApiBuilder;
use crate::util::{http_get_image, MessageChain};
use walle::walle_core::event::GroupMessageEvent;
use walle::walle_core::prelude::{Value, ValueMapExt};
use walle::walle_core::WalleResult;
use walle::{matcher, on_command, ActionCallerExt, MatcherHandler, Session};
use crate::seg_to_vec;

on_command!(SetuPlugin,setu => "/色图",setu_r18 => "/色图r");
/// 垃圾代码,有时间重构一下
pub struct SetuPluginMatcher;

impl SetuPluginMatcher {
    pub async fn setu(
        &self,
        ro: SetuPlugin,
        event: GroupMessageEvent,
        s: Session,
    ) -> WalleResult<()> {
        match ro {
            SetuPlugin::setu(seg) => {
                let (is_array, api) = if seg.is_empty() {
                    (
                        false,
                        LoliconApiBuilder::new().no_r18().exclude_ai().build(),
                    )
                } else {
                    let mut vec = seg_to_vec!(seg);
                    match vec.first().unwrap().parse::<i8>() {
                        Ok(data) => {
                            vec.remove(0);
                            if vec.is_empty() {
                                (
                                    false,
                                    LoliconApiBuilder::new()
                                        .no_r18()
                                        .num(data)
                                        .exclude_ai()
                                        .build(),
                                )
                            } else {
                                (
                                    true,
                                    LoliconApiBuilder::new()
                                        .no_r18()
                                        .num(data)
                                        .exclude_ai()
                                        .tag(vec)
                                        .build(),
                                )
                            }
                        }
                        Err(_) => (
                            true,
                            LoliconApiBuilder::new()
                                .no_r18()
                                .exclude_ai()
                                .tag(vec)
                                .build(),
                        ),
                    }
                };
                if is_array {
                    match LoliconApiBuilder::get_array(&api).await {
                        Ok(lolicon_struct_array) => {
                            for lolicon_struct in lolicon_struct_array {
                                match http_get_image(lolicon_struct.urls.original.as_str()).await {
                                    Ok(data) => {
                                        let message = MessageChain::new()
                                            .text(&format!("title: {}\n", lolicon_struct.title))
                                            .text(&format!("pid: {}\n", lolicon_struct.pid))
                                            .text(&format!("author: {}\n", lolicon_struct.author))
                                            .image_bytes(data)
                                            .build();
                                        match s.reply(message).await {
                                            Ok(smr) => {
                                                delete_message(&s, smr.message_id).await;
                                            }
                                            Err(_) => {
                                                s.reply("色图发送失败了喵...").await?;
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        s.reply("色图发送失败了喵...").await?;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            s.reply("获取api错误喵...").await?;
                        }
                    }
                } else {
                    match LoliconApiBuilder::get(&api).await {
                        Ok(lolicon_struct) => {
                            match http_get_image(lolicon_struct.urls.original.as_str()).await {
                                Ok(data) => {
                                    let message = MessageChain::new()
                                        .text(&format!("title: {}\n", lolicon_struct.title))
                                        .text(&format!("pid: {}\n", lolicon_struct.pid))
                                        .text(&format!("author: {}\n", lolicon_struct.author))
                                        .image_bytes(data)
                                        .build();
                                    match s.reply(message).await {
                                        Ok(smr) => {
                                            delete_message(&s, smr.message_id).await;
                                        }
                                        Err(_) => {
                                            s.reply("色图发送失败了喵...").await?;
                                        }
                                    }
                                }
                                Err(_) => {
                                    s.reply("色图发送失败了喵...").await?;
                                }
                            }
                        }
                        Err(_) => {
                            s.reply("获取api错误喵...").await?;
                        }
                    }
                }
            }
            SetuPlugin::setu_r18(seg) => {
                let (is_array, api) = if seg.is_empty() {
                    (false, LoliconApiBuilder::new().r18().build())
                } else {
                    let mut vec = seg_to_vec!(seg);
                    match vec.first().unwrap().parse::<i8>() {
                        Ok(data) => {
                            vec.remove(0);
                            if vec.is_empty() {
                                (false, LoliconApiBuilder::new().r18().num(data).build())
                            } else {
                                (
                                    true,
                                    LoliconApiBuilder::new().r18().num(data).tag(vec).build(),
                                )
                            }
                        }
                        Err(_) => (true, LoliconApiBuilder::new().r18().tag(vec).build()),
                    }
                };
                if is_array {
                    match LoliconApiBuilder::get_array(&api).await {
                        Ok(lolicon_struct_array) => {
                            for lolicon_struct in lolicon_struct_array {
                                match http_get_image(lolicon_struct.urls.original.as_str()).await {
                                    Ok(data) => {
                                        let message = MessageChain::new()
                                            .text(&format!("title: {}\n", lolicon_struct.title))
                                            .text(&format!("pid: {}\n", lolicon_struct.pid))
                                            .text(&format!("author: {}\n", lolicon_struct.author))
                                            .image_bytes(data)
                                            .build();
                                        match s.reply(message).await {
                                            Ok(smr) => {
                                                delete_message(&s, smr.message_id).await;
                                            }
                                            Err(_) => {
                                                s.reply("色图发送失败了喵...").await?;
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        s.reply("色图发送失败了喵...").await?;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            s.reply("获取api错误喵...").await?;
                        }
                    }
                } else {
                    match LoliconApiBuilder::get(&api).await {
                        Ok(lolicon_struct) => {
                            match http_get_image(lolicon_struct.urls.original.as_str()).await {
                                Ok(data) => {
                                    let message = MessageChain::new()
                                        .text(&format!("title: {}\n", lolicon_struct.title))
                                        .text(&format!("pid: {}\n", lolicon_struct.pid))
                                        .text(&format!("author: {}\n", lolicon_struct.author))
                                        .image_bytes(data)
                                        .build();
                                    match s.reply(message).await {
                                        Ok(smr) => {
                                            delete_message(&s, smr.message_id).await;
                                        }
                                        Err(_) => {
                                            s.reply("色图发送失败了喵...").await?;
                                        }
                                    }
                                }
                                Err(_) => {
                                    s.reply("色图发送失败了喵...").await?;
                                }
                            }
                        }
                        Err(_) => {
                            s.reply("获取api错误喵...").await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

async fn delete_message(s: &Session, message_id: String) {
    let s = s.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        s.delete_message(message_id).await.unwrap();
    });
}

matcher!(
    failable: SetuPluginMatcher,
    setu,
    ro: SetuPlugin,
    event: GroupMessageEvent
);

pub fn setu() -> impl MatcherHandler {
    std::sync::Arc::new(SetuPluginMatcher)
}
