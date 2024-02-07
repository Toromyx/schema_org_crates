/// <https://schema.org/FoodEstablishmentReservation>
pub trait FindFoodEstablishmentReservationIds {
	type IdType;
	/// <https://schema.org/FoodEstablishmentReservation>
	fn find_food_establishment_reservation_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFoodEstablishmentReservationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_food_establishment_reservation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTPS
				}
			})
		}
	}
}
