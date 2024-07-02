use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Character {
    pub char_id: u32,            // `int(11) unsigned NOT NULL AUTO_INCREMENT`
    pub account_id: u32,         // `int(11) unsigned NOT NULL DEFAULT 0`
    pub char_num: i8,            // `tinyint(1) NOT NULL DEFAULT 0`
    pub name: String,            // `varchar(30) NOT NULL DEFAULT ''`
    pub class: u16,              // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub base_level: u16,         // `smallint(6) unsigned NOT NULL DEFAULT 1`
    pub job_level: u16,          // `smallint(6) unsigned NOT NULL DEFAULT 1`
    pub base_exp: u64,           // `bigint(20) unsigned NOT NULL DEFAULT 0`
    pub job_exp: u64,            // `bigint(20) unsigned NOT NULL DEFAULT 0`
    pub zeny: u32,               // `int(11) unsigned NOT NULL DEFAULT 50000`
    pub str: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub agi: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub vit: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub int: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub dex: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub luk: u16,                // `smallint(4) unsigned NOT NULL DEFAULT 0`
    pub max_hp: u32,             // `int(11) unsigned NOT NULL DEFAULT 0`
    pub hp: u32,                 // `int(11) unsigned NOT NULL DEFAULT 0`
    pub max_sp: u32,             // `int(11) unsigned NOT NULL DEFAULT 0`
    pub sp: u32,                 // `int(11) unsigned NOT NULL DEFAULT 0`
    pub status_point: u32,       // `int(11) unsigned NOT NULL DEFAULT 0`
    pub skill_point: u32,        // `int(11) unsigned NOT NULL DEFAULT 0`
    pub option: i32,             // `int(11) NOT NULL DEFAULT 0`
    pub karma: i8,               // `tinyint(3) NOT NULL DEFAULT 0`
    pub manner: i16,             // `smallint(6) NOT NULL DEFAULT 0`
    pub party_id: u32,           // `int(11) unsigned NOT NULL DEFAULT 0`
    pub guild_id: u32,           // `int(11) unsigned NOT NULL DEFAULT 0`
    pub pet_id: u32,             // `int(11) unsigned NOT NULL DEFAULT 0`
    pub homun_id: u32,           // `int(11) unsigned NOT NULL DEFAULT 0`
    pub elemental_id: u32,       // `int(11) unsigned NOT NULL DEFAULT 0`
    pub hair: u8,                // `tinyint(4) unsigned NOT NULL DEFAULT 0`
    pub hair_color: u16,         // `smallint(5) unsigned NOT NULL DEFAULT 0`
    pub clothes_color: u16,      // `smallint(5) unsigned NOT NULL DEFAULT 0`
    pub body: u16,               // `smallint(5) unsigned NOT NULL DEFAULT 0`
    pub weapon: u16,             // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub shield: u16,             // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub head_top: u16,           // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub head_mid: u16,           // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub head_bottom: u16,        // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub robe: u16,               // `smallint(6) unsigned NOT NULL DEFAULT 0`
    pub last_map: String,        // `varchar(11) NOT NULL DEFAULT ''`
    pub last_x: u16,             // `smallint(4) unsigned NOT NULL DEFAULT 53`
    pub last_y: u16,             // `smallint(4) unsigned NOT NULL DEFAULT 111`
    pub save_map: String,        // `varchar(11) NOT NULL DEFAULT ''`
    pub save_x: u16,             // `smallint(4) unsigned NOT NULL DEFAULT 53`
    pub save_y: u16,             // `smallint(4) unsigned NOT NULL DEFAULT 111`
    pub partner_id: u32,         // `int(11) unsigned NOT NULL DEFAULT 0`
    pub online: i8,              // `tinyint(2) NOT NULL DEFAULT 0`
    pub father: u32,             // `int(11) unsigned NOT NULL DEFAULT 0`
    pub mother: u32,             // `int(11) unsigned NOT NULL DEFAULT 0`
    pub child: u32,              // `int(11) unsigned NOT NULL DEFAULT 0`
    pub fame: u32,               // `int(11) unsigned NOT NULL DEFAULT 0`
    pub rename: u16,             // `smallint(3) unsigned NOT NULL DEFAULT 0`
    pub delete_date: u32,        // `int(11) unsigned NOT NULL DEFAULT 0`
    pub moves: u32,              // `int(11) unsigned NOT NULL DEFAULT 0`
    pub unban_time: u32,         // `int(11) unsigned NOT NULL DEFAULT 0`
    pub font: u8,                // `tinyint(3) unsigned NOT NULL DEFAULT 0`
    pub uniqueitem_counter: u32, // `int(11) unsigned NOT NULL DEFAULT 0`
    pub sex: String,             // `enum('M','F') NOT NULL`
    pub hotkey_rowshift: u8,     // `tinyint(3) unsigned NOT NULL DEFAULT 0`
    pub hotkey_rowshift2: u8,    // `tinyint(3) unsigned NOT NULL DEFAULT 0`
    pub clan_id: u32,            // `int(11) unsigned NOT NULL DEFAULT 0`
    pub last_login: Option<PrimitiveDateTime>, // `datetime DEFAULT NULL`
    pub title_id: u32,           // `int(11) unsigned NOT NULL DEFAULT 0`
    pub show_equip: u8,          // `tinyint(3) unsigned NOT NULL DEFAULT 0`
}

impl Default for Character {
    fn default() -> Self {
        Self {
            char_id: 0,
            account_id: 0,
            char_num: 0,
            name: String::new(),
            class: 0,
            base_level: 1,
            job_level: 1,
            base_exp: 0,
            job_exp: 0,
            zeny: 50000,
            str: 0,
            agi: 0,
            vit: 0,
            int: 0,
            dex: 0,
            luk: 0,
            max_hp: 0,
            hp: 0,
            max_sp: 0,
            sp: 0,
            status_point: 0,
            skill_point: 0,
            option: 0,
            karma: 0,
            manner: 0,
            party_id: 0,
            guild_id: 0,
            pet_id: 0,
            homun_id: 0,
            elemental_id: 0,
            hair: 0,
            hair_color: 0,
            clothes_color: 0,
            body: 0,
            weapon: 0,
            shield: 0,
            head_top: 0,
            head_mid: 0,
            head_bottom: 0,
            robe: 0,
            last_map: String::new(),
            last_x: 53,
            last_y: 111,
            save_map: String::new(),
            save_x: 53,
            save_y: 111,
            partner_id: 0,
            online: 0,
            father: 0,
            mother: 0,
            child: 0,
            fame: 0,
            rename: 0,
            delete_date: 0,
            moves: 0,
            unban_time: 0,
            font: 0,
            uniqueitem_counter: 0,
            sex: String::from("M"),
            hotkey_rowshift: 0,
            hotkey_rowshift2: 0,
            clan_id: 0,
            last_login: None,
            title_id: 0,
            show_equip: 0,
        }
    }
}
