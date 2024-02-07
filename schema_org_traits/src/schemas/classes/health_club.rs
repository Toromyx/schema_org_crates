/// <https://schema.org/HealthClub>
pub trait FindHealthClubIds {
	type IdType;
	/// <https://schema.org/HealthClub>
	fn find_health_club_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHealthClubIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_health_club_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HEALTH_CLUB_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HEALTH_CLUB_IRI_HTTPS,
			})
		}
	}
}
