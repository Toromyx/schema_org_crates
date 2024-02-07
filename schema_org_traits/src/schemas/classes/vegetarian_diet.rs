/// <https://schema.org/VegetarianDiet>
pub trait FindVegetarianDietIds {
	type IdType;
	/// <https://schema.org/VegetarianDiet>
	fn find_vegetarian_diet_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVegetarianDietIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_vegetarian_diet_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VEGETARIAN_DIET_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VEGETARIAN_DIET_IRI_HTTPS,
			})
		}
	}
}
