/// <https://schema.org/AdultEntertainment>
pub trait FindAdultEntertainmentIds {
	type IdType;
	fn find_adult_entertainment_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAdultEntertainmentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_adult_entertainment_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ADULT_ENTERTAINMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ADULT_ENTERTAINMENT_IRI_HTTPS,
			})
		}
	}
}
