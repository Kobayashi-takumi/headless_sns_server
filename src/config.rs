use anyhow::Result;
use std::env::var;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppConfig {
    pub api_domain: String,
    pub api_port: u16,
    pub db_path: String,
    pub db_user: String,
    pub db_password: String,
    pub db_ns: String,
    pub db_name: String,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let api_domain = var("API_DOMAIN").unwrap();
        let api_port = var("API_PORT").unwrap().parse()?;
        let db_path = var("DB_PATH").unwrap();
        let db_user = var("DB_USER").unwrap();
        let db_password = var("DB_PASSWORD").unwrap();
        let db_ns = var("DB_NS").unwrap();
        let db_name = var("DB_NAME").unwrap();
        println!(
            r#"
API_DOMAIN: {api_domain}
API_PORT:   {api_port}
DB_PATH:    {db_path}
DB_USER:    {db_user}
DB_PASS:    {db_password}
DB_NS:      {db_ns}
DB_NAME:    {db_name}
        "#
        );
        Ok(Self {
            api_domain,
            api_port,
            db_path,
            db_user,
            db_password,
            db_ns,
            db_name,
        })
    }
}
