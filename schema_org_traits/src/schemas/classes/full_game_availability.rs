/// <https://schema.org/FullGameAvailability>
pub trait FindFullGameAvailabilityIds {
	type IdType;
	/// <https://schema.org/FullGameAvailability>
	fn find_full_game_availability_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFullGameAvailabilityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_full_game_availability_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FULL_GAME_AVAILABILITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FULL_GAME_AVAILABILITY_IRI_HTTPS,
			})
		}
	}
}
