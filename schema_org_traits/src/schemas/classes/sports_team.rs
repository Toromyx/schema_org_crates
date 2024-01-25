/// <https://schema.org/SportsTeam>
pub trait FindSportsTeamIds {
	type IdType;
	fn find_sports_team_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSportsTeamIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sports_team_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SPORTS_TEAM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SPORTS_TEAM_IRI_HTTPS,
			})
		}
	}
}
