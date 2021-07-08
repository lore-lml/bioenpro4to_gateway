use std::sync::Mutex;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use iota_identity_lib::api::IdentityManager;

#[derive(Debug)]
pub struct EnvConfig{
    host_address: String,
    port: u16,
    root_channel_psw: String,
    identity_issuer_name: String,
    mainnet: bool
}

impl EnvConfig{
    pub fn from_env() -> anyhow::Result<EnvConfig>{
        dotenv::dotenv().ok();
        let host_address = {
            let addr = dotenv::var("HOST_ADDRESS")?;
            if addr == "localhost"{
                "127.0.0.1".to_owned()
            }else {
                addr
            }
        };
        let port = dotenv::var("PORT")?.parse::<u16>()?;
        let root_channel_psw = dotenv::var("ROOT_CHANNEL_PSW")?;
        let identity_issuer_name = dotenv::var("IDENTITY_ISSUER_NAME")?;
        let mainnet = dotenv::var("MAINNET")?.parse()?;
        Ok(EnvConfig{host_address, port, root_channel_psw, identity_issuer_name, mainnet})
    }

    pub fn address(&self) -> String{
        format!("{}:{}", self.host_address, self.port)
    }
    pub fn root_channel_psw(&self) -> &str {
        &self.root_channel_psw
    }
    pub fn identity_issuer_name(&self) -> &str {
        &self.identity_issuer_name
    }
    pub fn url(&self) -> String{
        let host = {
            if self.host_address == "0.0.0.0"{
                "192.168.1.91"
            }else if self.host_address == "127.0.0.1"{
                "localhost"
            }else{
                &self.host_address
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
        let root = RootChannel::new(config.mainnet);
        let identity = IdentityManager::default().await?;
        let mut state = AppState { root: Mutex::new(root), identity: Mutex::new(identity), config };
        state.init().await?;
        Ok(state)
    }
    async fn init(&mut self) -> anyhow::Result<()>{
        let info = self.root.lock().unwrap()
            .open(self.config.root_channel_psw()).await?;
        println!("Root Channel -> https://streams-chrysalis-explorer.netlify.app/channel/{}:{}?mainnet={}",
                 info.channel_id(), info.announce_id(), self.config.mainnet);
        let did = self.identity.lock().unwrap()
            .create_identity(self.config.identity_issuer_name()).await?.id().as_str().to_string();
        println!("{} DID: {}", self.config.identity_issuer_name(), did);
        Ok(())
    }
}
