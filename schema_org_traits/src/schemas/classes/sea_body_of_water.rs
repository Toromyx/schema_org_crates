/// <https://schema.org/SeaBodyOfWater>
pub trait FindSeaBodyOfWaterIds {
	type IdType;
	fn find_sea_body_of_water_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSeaBodyOfWaterIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sea_body_of_water_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SEA_BODY_OF_WATER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SEA_BODY_OF_WATER_IRI_HTTPS,
			})
		}
	}
}
