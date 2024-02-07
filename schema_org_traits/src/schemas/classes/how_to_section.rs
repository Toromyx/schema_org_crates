/// <https://schema.org/HowToSection>
pub trait FindHowToSectionIds {
	type IdType;
	/// <https://schema.org/HowToSection>
	fn find_how_to_section_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHowToSectionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_how_to_section_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HOW_TO_SECTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HOW_TO_SECTION_IRI_HTTPS,
			})
		}
	}
}
