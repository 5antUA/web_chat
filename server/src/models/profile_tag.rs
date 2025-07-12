use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::profiles_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProfileTag {
    fk_profile_id: i32,
    fk_tag_id: i32,
    created_at: NaiveDateTime,
}
