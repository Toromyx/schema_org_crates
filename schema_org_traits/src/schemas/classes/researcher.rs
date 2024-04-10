/// <https://schema.org/Researcher>
pub trait FindResearcherIds {
	type IdType;
	/// <https://schema.org/Researcher>
	fn find_researcher_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindResearcherIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_researcher_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESEARCHER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESEARCHER_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindResearcherIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_researcher_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESEARCHER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESEARCHER_IRI_HTTPS,
			})
		}
	}
}
