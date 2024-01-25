/// <https://schema.org/PoliticalParty>
pub trait FindPoliticalPartyIds {
	type IdType;
	fn find_political_party_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPoliticalPartyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_political_party_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::POLITICAL_PARTY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::POLITICAL_PARTY_IRI_HTTPS,
			})
		}
	}
}
