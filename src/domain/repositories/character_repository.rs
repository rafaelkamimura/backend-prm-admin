use crate::domain::models::character::Character;
use async_trait::async_trait;

#[async_trait]
pub trait CharacterRepository: Send + Sync {
    async fn get_all_characters(&self) -> Result<Vec<Character>, sqlx::Error>;
    async fn get_online_characters(&self) -> Result<Vec<Character>, sqlx::Error>;
}
