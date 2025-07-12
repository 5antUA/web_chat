use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    id: i32,
    fk_user_id: i32,
    age: i32,
    bio: Option<String>,
    avatar_url: Option<String>,
    created_at: NaiveDateTime,
}
