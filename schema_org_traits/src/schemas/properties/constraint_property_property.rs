/// <https://schema.org/constraintProperty>
pub trait GetConstraintPropertyProperty {
	type IdType;
	type PropertyType;
	fn get_constraint_property_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetConstraintPropertyProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_constraint_property_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
