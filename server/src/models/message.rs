use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    id: i32,
    fk_user_id: i32,
    message_content: Option<String>,
    created_at: NaiveDateTime,
}
