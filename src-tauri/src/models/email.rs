use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub sender_email: String,
    pub sender_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplateData {
    pub subject: String,
    pub body: String,
}
