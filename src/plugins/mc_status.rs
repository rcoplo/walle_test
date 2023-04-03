use chrono::Timelike;
use walle::{ActionCallerExt, matcher, MatcherHandler, on_command, Session};
use walle::walle_core::event::GroupMessageEvent;

use walle::walle_core::WalleResult;
use crate::{CONTEXT, seg_to_vec, string_to_i64};
use crate::api::mc_status_api::{get_minecraft_status_bedrock, get_minecraft_status_java};
use crate::database::implement::mc_server_impl::McServerType;

use crate::util::{MessageChain, ToNum};

on_command!(McStatusPlugin,Mc => "/mc",list => "/list");

pub struct McStatusPluginMatcher;

impl McStatusPluginMatcher {
    pub async fn mc_status(
        &self,
        msp: McStatusPlugin,
        event: GroupMessageEvent,
        s: Session,
    ) -> WalleResult<()> {
        match msp {
            McStatusPlugin::Mc(seg) => {
                //sdk暂时未提供获取群管理权限的api,暂时只允许设置得super_admin使用该指令
                //let v_user = s.get_group_member_list(event.detail_type.group_id).await?;
               if regex::RegexSet::new(CONTEXT.config.bot_config.super_admin.iter()).unwrap().is_match(event.ty.user_id.as_str()){
                   let group_id = string_to_i64!(event.detail_type.group_id);
                   let v = seg_to_vec!(seg);
                   if v.is_empty() {
                       s.reply(
                           MessageChain::new()
                               .text("可用子指令:\n")
                               .text(">    add\n")
                               .text(">    upname\n")
                               .text(">    upurl\n")
                               .text(">    uptype\n")
                               .text(">    d\n")
                               .text("此命令现仅超级管理员可用")
                               .build()
                       ).await?;
                   }else {
                       walle::tracing::info!("{:?}",v);
                       walle::tracing::info!("{:?}",v.first());
                       match v.first().unwrap_or(&"".to_string()).as_str() {
                           "add" => {
                               if let (Some(name),Some(url)) =  (v.get(1),v.get(2)){
                                   match CONTEXT.mc_server.new(name.to_uppercase().as_str(), url, group_id, Ok(McServerType::JAVA)).await {
                                       Ok(_) => {
                                           s.reply("添加服务器成功喵!").await?;
                                       }
                                       Err(err) => {
                                           s.reply(err.to_msg_segment()).await?;
                                       }
                                   }
                               }else {
                                   s.reply("参数不够喵...,\n 指令: /mc add {name} {Address}").await?;
                               }
                           }
                           "upname" =>{
                               if let (Some(name),Some(new_name)) =  (v.get(1),v.get(2)){
                                   match CONTEXT.mc_server.update_name_by_name_group_id(name.to_uppercase().as_str(),group_id, new_name.to_uppercase().as_str()).await {
                                       Ok(_) => {
                                           s.reply("修改服务器简称成功喵!").await?;
                                       }
                                       Err(err) => {
                                           s.reply(err.to_msg_segment()).await?;
                                       }
                                   }
                               }else {
                                   s.reply("参数不够喵...,\n 指令: /mc upname {name} {new_name}").await?;
                               }

                           }
                           "upurl" =>{
                               if let (Some(name),Some(new_url)) =  (v.get(1),v.get(2)){
                                   match CONTEXT.mc_server.update_url_by_name_group_id(name.to_uppercase().as_str(),group_id, new_url.as_str()).await {
                                       Ok(_) => {
                                           s.reply("修改服务器url成功喵!").await?;
                                       }
                                       Err(err) => {
                                           s.reply(err.to_msg_segment()).await?;
                                       }
                                   }
                               }else {
                                   s.reply("参数不够喵...,\n 指令: /mc upname {name} {new_url}").await?;
                               }
                           }
                           "uptype" =>{
                               if let (Some(name),Some(new_type)) =  (v.get(1),v.get(2)){
                                   match CONTEXT.mc_server.update_server_type_by_name_group_id(name.to_uppercase().as_str(),group_id, McServerType::new(new_type.to_uppercase().as_str())).await {
                                       Ok(_) => {
                                           s.reply("修改服务器type成功喵!").await?;
                                       }
                                       Err(err) => {
                                           s.reply(err.to_msg_segment()).await?;
                                       }
                                   }
                               }else {
                                   s.reply("参数不够喵...,\n 指令: /mc uptype {name} {new_type} \nnew_type可用参数:[JE,BE]").await?;
                               }
                           }
                           "d" =>{
                               if let Some(name) =  v.get(1){
                                   match CONTEXT.mc_server.delete_server_by_name_group_id(name.to_uppercase().as_str(),group_id).await {
                                       Ok(_) => {
                                           s.reply("删除服务器成功喵!").await?;
                                       }
                                       Err(err) => {
                                           s.reply(err.to_msg_segment()).await?;
                                       }
                                   }
                               }else {
                                   s.reply("参数不够喵...,\n 指令: /mc d {name}").await?;
                               }
                           }
                           _ => {
                               s.reply("没有这个子指令喵...").await?;
                           }
                       }
                   }
                }else {
                   s.reply("你没有权限使用该指令喵...").await?;
               }
            }
            McStatusPlugin::list(seg) => {
                let v = seg_to_vec!(seg);
                if v.is_empty() {
                    match CONTEXT.mc_server.select_server_all_by_group_id(event.detail_type.group_id.to_i64()).await {
                        None => {}
                        Some(v) => {
                            if v.is_empty(){
                                s.reply("本群没有绑定服务器喵...").await?;
                                return Ok(());
                            }
                            let mut chain = MessageChain::new();
                            chain.text("当前可用服务器列表:\n");
                            for (i,server) in v.iter().enumerate() {
                                chain.text(&format!("{}. {}\n",i+1,server.name));
                            }
                            chain.text("输入/list {name}获取在线信息");
                            s.reply(chain.build()).await?;
                        }
                    }
                }else {
                    if let Some(name) =v.first(){
                        match CONTEXT.mc_server.select_server_by_name_group_id(name.to_uppercase().as_str(), event.detail_type.group_id.to_i64()).await {
                            None => {
                                s.reply("本群并没有这个服务器简称喵...").await?;
                            }
                            Some(mc_server) => {
                                match McServerType::new(mc_server.server_type.as_str()) {
                                    Ok(server_type) => {
                                        match server_type {
                                            McServerType::JAVA => {
                                                match get_minecraft_status_java(mc_server.url.as_str()).await {
                                                    Ok(status) => {
                                                        if status.online {
                                                            let mut chain = MessageChain::new();
                                                            chain.text(&format!("{} Online: {}/{}\n", mc_server.name, status.players.online, status.players.max));
                                                            let vec = status.players.list
                                                                .iter()
                                                                .map(|list| {
                                                                    list.name_raw.to_owned()
                                                                }).collect::<Vec<_>>();
                                                            if vec.len() == 0 {
                                                                chain.text("没有玩家在服务器喵...");
                                                            } else {
                                                                chain.text(&format!("{:?}", vec)
                                                                    .replace("\"", "")
                                                                    .replace("[", "")
                                                                    .replace("]", ""));
                                                            }
                                                            let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                                            let time = chrono::Utc::now().naive_utc();
                                                            if cache_time.minute() > time.minute() {
                                                                let duration = cache_time.time() - time.time();
                                                                if duration.num_seconds() < 55 {
                                                                    chain.text(&format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                                }
                                                            }
                                                            s.reply(chain.build()).await?;
                                                        } else {
                                                            s.reply("服务器当前不在线喵...").await?;
                                                        }
                                                    }
                                                    Err(err) => {
                                                        s.reply(err.to_msg_segment()).await?;
                                                    }
                                                }
                                            }
                                            McServerType::Bedrock => {
                                                match get_minecraft_status_bedrock(mc_server.url.as_str()).await {
                                                    Ok(status) => {
                                                        if status.online {
                                                            let mut chain = MessageChain::new();
                                                            chain.text(&format!("{} Players: {}/{}\n", mc_server.name, status.players.online, status.players.max));
                                                            chain.text("Bedrock版无法获取到玩家列表喵!");
                                                            let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                                            let time = chrono::Utc::now().naive_utc();
                                                            if cache_time.minute() > time.minute() {
                                                                let duration = cache_time.time() - time.time();
                                                                if duration.num_seconds() < 55 {
                                                                    chain.text(&format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                                }
                                                            }
                                                            s.reply(chain.build()).await?;
                                                        } else {
                                                            s.reply("服务器当前不在线喵...").await?;

                                                        }
                                                    }
                                                    Err(err) => {
                                                        s.reply(err.to_msg_segment()).await?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        s.reply(err.to_msg_segment()).await?;
                                    }
                                }
                            }
                        }

                    }
                }
            }
        }
        Ok(())
    }
}

matcher!(
    failable: McStatusPluginMatcher,
    mc_status,
    msp: McStatusPlugin,
    event: GroupMessageEvent
);

pub fn mc_status() -> impl MatcherHandler {
    std::sync::Arc::new(McStatusPluginMatcher)
}