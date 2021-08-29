use std::sync::Mutex;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use iota_identity_lib::api::{IdentityManager, Storage};
use bioenpro4to_channel_manager::channels::{ChannelInfo, ActorChannelInfo, Category};
use regex::Regex;
use crate::utils::message_cache::MessageCache;
use bioenpro4to_channel_manager::utils::{timestamp_to_date_string, current_time_secs};

const DEFAULT_PSW: &str = "zH!rRAtmODw*W$k4%0MxuRez^BQQsp";

pub struct EnvConfig{
    server_addr: String,
    server_port: u16,
    mainnet: bool,
    root_channel_addr: Option<ChannelInfo>,
    root_channel_psw: String,
    msgs_update_time: i64,
    identity_issuer_name: String,
    storage: Storage,
}

#[allow(dead_code)]
impl EnvConfig{
    pub fn from_env() -> anyhow::Result<EnvConfig>{
        let server_addr = {
            let addr = dotenv::var("SERVER.ADDR").map_or("127.0.0.1".to_owned(), |x| x);
            if addr == "localhost"{
                "127.0.0.1".to_owned()
            }else {
                addr
            }
        };
        let server_port: u16 = dotenv::var("SERVER.PORT").map_or(8080, |x| x.parse().map_or(8080, |y| y));

        let mainnet = dotenv::var("IOTA.MAINNET").map_or(false, |x| x.parse().map_or(false, |y| y));
        let root_channel_addr = dotenv::var("IOTA.ROOT_CHANNEL.ADDR").map_or(None, |x| {
            let re = Regex::new(r".+:.+").unwrap();
            if re.is_match(&x){
                let vec: Vec<&str> = x.split(":").collect();
                Some(ChannelInfo::new(vec[0].to_string(), vec[1].to_string()))
            }else{
                eprintln!("IOTA.ROOT_CHANNEL.ADDR bad format: expected <channel_id>:<announce_id>");
                eprintln!("Creating new root channel ...");
                None
            }
        });
        let root_channel_psw = dotenv::var("IOTA.ROOT_CHANNEL.PSW").map_or(DEFAULT_PSW.to_owned(), |x| x);
        let msgs_update_time: i64 = dotenv::var("IOTA.MESSAGES_UPDATE_TIME").map_or(1, |x| x.parse().map_or(1, |y| y));

        let identity_issuer_name = dotenv::var("IOTA.IDENTITY.ISSUER_NAME").map_or("unknown".to_owned(), |x| x);
        let storage= dotenv::var("IOTA.IDENTITY.STORAGE.TYPE").map_or(Storage::Memory, |x| {
            match x.as_str(){
                "stronghold" => {
                    let dir = dotenv::var("IOTA.IDENTITY.STORAGE.DIR").map_or("./state".to_owned(), |x| x);
                    let psw = dotenv::var("IOTA.IDENTITY.STORAGE.PSW").map_or(DEFAULT_PSW.to_owned(), |x| x);
                    Storage::Stronghold(dir, Some(psw))
                }
                _ => Storage::Memory,
            }
        });

        Ok(EnvConfig{
            server_addr, server_port,
            mainnet, root_channel_addr, root_channel_psw, msgs_update_time,
            identity_issuer_name, storage,
        })
    }

    pub fn address(&self) -> String{
        format!("{}:{}", self.server_addr, self.server_port)
    }
    pub fn root_channel_psw(&self) -> &str {
        &self.root_channel_psw
    }
    pub fn identity_issuer_name(&self) -> &str {
        &self.identity_issuer_name
    }
    pub fn is_main_net(&self) -> bool{
        self.mainnet
    }
    pub fn url(&self) -> String{
        let host = {
            if self.server_addr == "0.0.0.0"{
                "192.168.1.91"
            }else if self.server_addr == "127.0.0.1"{
                "localhost"
            }else{
                &self.server_addr
            }
        };
        format!("http://{}:{}", host, self.server_port)
    }
}

pub struct AppState{
    pub root: Mutex<RootChannel>,
    pub identity: Mutex<IdentityManager>,
    pub msg_cache: Mutex<MessageCache>,
    pub config: EnvConfig,
}

impl AppState{
    pub async fn from_config(config: EnvConfig) -> anyhow::Result<Self> {
        let (root, open) = match &config.root_channel_addr{
            None => (RootChannel::new(config.mainnet), true),
            Some(info) => (
                RootChannel::import_from_tangle(
                info.channel_id(),
                info.announce_id(),
                config.root_channel_psw(), config.mainnet
                ).await?,
                false
            )
        };

        let identity = IdentityManager::builder()
            .storage(config.storage.clone())
            .main_net(config.mainnet)
            .build()
            .await?;
        let mut state = AppState {
            root: Mutex::new(root),
            identity: Mutex::new(identity),
            msg_cache: Mutex::new(MessageCache::new(config.mainnet, config.msgs_update_time)),
            config
        };
        state.init(open).await?;
        Ok(state)
    }
    async fn init(&mut self, open: bool) -> anyhow::Result<()>{
        let info = if open {
            self.root.lock().unwrap().open(self.config.root_channel_psw()).await?
        } else {
            self.root.lock().unwrap().channel_info()
        };

        self.cache_daily_messages().await?;

        println!("Root Channel -> https://streams-chrysalis-explorer.netlify.app/channel/{}:{}?mainnet={}",
                 info.channel_id(), info.announce_id(), self.config.mainnet);

        let mut manager = self.identity.lock().unwrap();
        let did = match manager.get_identity(self.config.identity_issuer_name()){
            None => manager.create_identity(self.config.identity_issuer_name()).await?.id().as_str().to_string(),
            Some(doc) => doc.id().as_str().to_string()
        };

        println!("{} DID: {}", self.config.identity_issuer_name(), did);
        Ok(())
    }

    async fn cache_daily_messages(&mut self) -> anyhow::Result<()>{
        let current_date = timestamp_to_date_string(current_time_secs(), false);
        println!("Caching Messages of date {}", current_date);
        let root = self.root.lock().unwrap();
        let actors: Vec<ActorChannelInfo> = [Category::Trucks, Category::Scales, Category::BioCells].iter()
            .flat_map(|category| {
                root.actors_of_category(category.clone())
            }).collect();

        let daily_channels = actors.iter()
            .flat_map(|a| {
                let category = Category::from_string(a.category()).unwrap();
                root.channels_of_actor(category, a.actor_id())
                    .into_iter().filter(|ch| ch.creation_date() == current_date)
                    .map(|ch| ch.address().clone())
                    .collect::<Vec<ChannelInfo>>()
            })
            .collect::<Vec<ChannelInfo>>();

        let mut found = 0;
        let mut cache = self.msg_cache.lock().unwrap();
        for ch in daily_channels {
            found += cache.get(&ch.to_string()).await?.len();
        }
        println!("  Caching complete: {} messages found", found);
        Ok(())
    }
}
