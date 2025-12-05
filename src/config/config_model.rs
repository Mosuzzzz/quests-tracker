#[derive(Debug,Clone)]
pub struct DotenvConfig {
    pub server:Server,
    pub database:Database,
}

#[derive(Debug,Clone)]
pub struct Server {
    pub port:u16,
    pub body_limit:usize,
    pub timeout:u64,
}

#[derive(Debug,Clone)]
pub struct Database {
    pub url:String,
}

#[derive(Debug,Clone)]
pub struct AdventurersSecret {
    pub secret:String,
    pub refresh_secret:String,
}

#[derive(Debug,Clone)]
pub struct GuildCommandersSecret {
    pub secret:String,
    pub refresh_secret:String,
}