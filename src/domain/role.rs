#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "role_type")]
#[sqlx(rename_all = "lowercase")]
enum Role {
    Organizer,
    Participant,
}
