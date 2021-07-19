use std::sync::Mutex;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use iota_identity_lib::api::{IdentityManager, Storage};
use bioenpro4to_channel_manager::channels::ChannelInfo;
use regex::Regex;

const DEFAULT_PSW: &str = "zH!rRAtmODw*W$k4%0MxuRez^BQQsp";

pub struct EnvConfig{
    server_addr: String,
    port: u16,
    root_channel_addr: Option<ChannelInfo>,
    root_channel_psw: String,
    identity_issuer_name: String,
    mainnet: bool,
    storage: Storage,
}

#[allow(dead_code)]
impl EnvConfig{
    pub fn from_env() -> anyhow::Result<EnvConfig>{
        let server_addr = {
            let addr = dotenv::var("SERVER_ADDR").map_or("127.0.0.1".to_owned(), |x| x);
            if addr == "localhost"{
                "127.0.0.1".to_owned()
            }else {
                addr
            }
        };
        let port: u16 = dotenv::var("PORT").map_or(8080, |x| x.parse().map_or(8080, |y| y));
        let root_channel_addr = dotenv::var("ROOT_CHANNEL_ADDR").map_or(None, |x| {
            let re = Regex::new(r".+:.+").unwrap();
            if re.is_match(&x){
                let vec: Vec<&str> = x.split(":").collect();
                Some(ChannelInfo::new(vec[0].to_string(), vec[1].to_string()))
            }else{
                eprintln!("ROOT_CHANNEL_ADDR bad format: expected <channel_id>:<announce_id>");
                eprintln!("Creating new root channel ...");
                None
            }
        });
        let root_channel_psw = dotenv::var("ROOT_CHANNEL_PSW").map_or(DEFAULT_PSW.to_owned(), |x| x);
        let identity_issuer_name = dotenv::var("IDENTITY_ISSUER_NAME").map_or("unknown".to_owned(), |x| x);
        let mainnet = dotenv::var("MAINNET").map_or(false, |x| x.parse().map_or(false, |y| y));
        let storage= dotenv::var("IDENTITY_STORAGE").map_or(Storage::Memory, |x| {
            match x.as_str(){
                "stronghold" => {
                    let dir = dotenv::var("IDENTITY_STORAGE_DIR").map_or("./state".to_owned(), |x| x);
                    let psw = dotenv::var("IDENTITY_STORAGE_PSW").map_or(DEFAULT_PSW.to_owned(), |x| x);
                    Storage::Stronghold(dir, Some(psw))
                }
                _ => Storage::Memory,
            }
        });

        Ok(EnvConfig{ server_addr, port, root_channel_addr, root_channel_psw, identity_issuer_name, mainnet, storage})
    }

    pub fn address(&self) -> String{
        format!("{}:{}", self.server_addr, self.port)
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
        format!("http://{}:{}", host, self.port)
    }
}

pub struct AppState{
    pub root: Mutex<RootChannel>,
    pub identity: Mutex<IdentityManager>,
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
        let mut state = AppState { root: Mutex::new(root), identity: Mutex::new(identity), config };
        state.init(open).await?;
        Ok(state)
    }
    async fn init(&mut self, open: bool) -> anyhow::Result<()>{
        let info = if open {
            self.root.lock().unwrap().open(self.config.root_channel_psw()).await?
        } else {
            self.root.lock().unwrap().channel_info()
        };
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
}
