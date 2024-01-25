/// <https://schema.org/DietarySupplement>
pub trait FindDietarySupplementIds {
	type IdType;
	fn find_dietary_supplement_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDietarySupplementIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dietary_supplement_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DIETARY_SUPPLEMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DIETARY_SUPPLEMENT_IRI_HTTPS,
			})
		}
	}
}
