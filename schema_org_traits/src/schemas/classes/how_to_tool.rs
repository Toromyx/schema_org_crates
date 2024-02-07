/// <https://schema.org/HowToTool>
pub trait FindHowToToolIds {
	type IdType;
	/// <https://schema.org/HowToTool>
	fn find_how_to_tool_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHowToToolIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_how_to_tool_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HOW_TO_TOOL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HOW_TO_TOOL_IRI_HTTPS,
			})
		}
	}
}
