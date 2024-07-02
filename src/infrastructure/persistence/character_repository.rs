use crate::domain::models::character::Character;
use crate::domain::repositories::character_repository::CharacterRepository;
use async_trait::async_trait;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct MySqlCharacterRepository {
    pool: MySqlPool,
}

impl MySqlCharacterRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CharacterRepository for MySqlCharacterRepository {
    async fn get_all_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Character,
            r#"
            SELECT char_id, account_id, char_num, name, class, base_level, job_level, base_exp,
                   job_exp, zeny, `str`, `agi`, `vit`, `int`, `dex`, `luk`, max_hp, hp, max_sp, sp, status_point,
                   skill_point, `option`, karma, manner, party_id, guild_id, pet_id, homun_id,
                   elemental_id, hair, hair_color, clothes_color, body, weapon, shield, head_top,
                   head_mid, head_bottom, robe, last_map, last_x, last_y, save_map, save_x, save_y,
                   partner_id, online, father, mother, child, fame, `rename`, delete_date, moves,
                   unban_time, font, uniqueitem_counter, sex, hotkey_rowshift, hotkey_rowshift2,
                   clan_id, last_login, title_id, show_equip
            FROM `char`
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_online_characters(&self) -> Result<Vec<Character>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Character,
            r#"
            SELECT char_id, account_id, char_num, name, class, base_level, job_level, base_exp,
                   job_exp, zeny, `str`, `agi`, `vit`, `int`, `dex`, `luk`, max_hp, hp, max_sp, sp, status_point,
                   skill_point, `option`, karma, manner, party_id, guild_id, pet_id, homun_id,
                   elemental_id, hair, hair_color, clothes_color, body, weapon, shield, head_top,
                   head_mid, head_bottom, robe, last_map, last_x, last_y, save_map, save_x, save_y,
                   partner_id, online, father, mother, child, fame, `rename`, delete_date, moves,
                   unban_time, font, uniqueitem_counter, sex, hotkey_rowshift, hotkey_rowshift2,
                   clan_id, last_login, title_id, show_equip
            FROM `char`
            WHERE online = 1
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
