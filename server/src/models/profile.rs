use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: i32,
    pub fk_user_id: i32,
    pub age: i32,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}
