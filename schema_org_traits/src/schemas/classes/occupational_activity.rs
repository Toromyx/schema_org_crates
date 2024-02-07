/// <https://schema.org/OccupationalActivity>
pub trait FindOccupationalActivityIds {
	type IdType;
	/// <https://schema.org/OccupationalActivity>
	fn find_occupational_activity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOccupationalActivityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_occupational_activity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OCCUPATIONAL_ACTIVITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OCCUPATIONAL_ACTIVITY_IRI_HTTPS,
			})
		}
	}
}
