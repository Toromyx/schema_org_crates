/// <https://schema.org/ItemAvailability>
pub trait FindItemAvailabilityIds {
	type IdType;
	/// <https://schema.org/ItemAvailability>
	fn find_item_availability_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindItemAvailabilityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_item_availability_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ITEM_AVAILABILITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ITEM_AVAILABILITY_IRI_HTTPS,
			})
		}
	}
}
