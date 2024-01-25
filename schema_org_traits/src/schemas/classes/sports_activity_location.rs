/// <https://schema.org/SportsActivityLocation>
pub trait FindSportsActivityLocationIds {
	type IdType;
	fn find_sports_activity_location_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSportsActivityLocationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sports_activity_location_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SPORTS_ACTIVITY_LOCATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::SPORTS_ACTIVITY_LOCATION_IRI_HTTPS
				}
			})
		}
	}
}
