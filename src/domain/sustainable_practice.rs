use serde::Deserialize;

#[derive(sqlx::Type, Debug, Clone, Copy, Deserialize)]
#[sqlx(type_name = "backend.sustainable_practice")]
pub enum SustainablePractice {
    RecycledMaterials,
    CollectiveTransport,
    RenewableEnergy,
    WasteReduction,
}
