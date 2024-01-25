/// <https://schema.org/Restaurant>
pub trait FindRestaurantIds {
	type IdType;
	fn find_restaurant_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRestaurantIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_restaurant_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESTAURANT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESTAURANT_IRI_HTTPS,
			})
		}
	}
}
