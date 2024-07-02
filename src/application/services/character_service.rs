use crate::domain::models::character::Character;
use crate::domain::repositories::character_repository::CharacterRepository;

pub struct CharacterService<R: CharacterRepository> {
    repository: R,
}

impl<R: CharacterRepository> CharacterService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        self.repository.get_all_characters().await
    }
    pub async fn get_online_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        self.repository.get_online_characters().await
    }
}

impl<R: CharacterRepository + Clone> Clone for CharacterService<R> {
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}
