/// <https://schema.org/PropertyValueSpecification>
pub trait FindPropertyValueSpecificationIds {
	type IdType;
	/// <https://schema.org/PropertyValueSpecification>
	fn find_property_value_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPropertyValueSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_property_value_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PROPERTY_VALUE_SPECIFICATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PROPERTY_VALUE_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
