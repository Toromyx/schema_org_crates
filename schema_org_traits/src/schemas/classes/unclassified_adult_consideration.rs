/// <https://schema.org/UnclassifiedAdultConsideration>
pub trait FindUnclassifiedAdultConsiderationIds {
	type IdType;
	fn find_unclassified_adult_consideration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUnclassifiedAdultConsiderationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_unclassified_adult_consideration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTPS
				}
			})
		}
	}
}
