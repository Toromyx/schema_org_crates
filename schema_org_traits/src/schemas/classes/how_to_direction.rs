/// <https://schema.org/HowToDirection>
pub trait FindHowToDirectionIds {
	type IdType;
	/// <https://schema.org/HowToDirection>
	fn find_how_to_direction_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHowToDirectionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_how_to_direction_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HOW_TO_DIRECTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HOW_TO_DIRECTION_IRI_HTTPS,
			})
		}
	}
}
