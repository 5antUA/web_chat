use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::profiles_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProfileTag {
    pub fk_profile_id: i32,
    pub fk_tag_id: i32,
    pub created_at: NaiveDateTime,
}
