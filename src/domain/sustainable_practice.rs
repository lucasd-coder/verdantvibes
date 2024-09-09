#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "role_type")]
#[sqlx(rename_all = "lowercase")]
enum SustainablePractice {
    RecycledMaterials,
    CollectiveTransport,
    RenewableEnergy,
    WasteReduction,
}
