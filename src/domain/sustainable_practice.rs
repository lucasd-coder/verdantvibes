use serde::Deserialize;

#[derive(sqlx::Type, Debug, Clone, Copy, Deserialize)]
#[sqlx(type_name = "sustainable_practice")]
// #[sqlx(rename_all = "lowercase")]
pub enum SustainablePractice {
    RecycledMaterials,
    CollectiveTransport,
    RenewableEnergy,
    WasteReduction,
}
