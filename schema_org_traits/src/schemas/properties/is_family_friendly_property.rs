/// <https://schema.org/isFamilyFriendly>
pub trait GetIsFamilyFriendlyProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/isFamilyFriendly>
	fn get_is_family_friendly_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetIsFamilyFriendlyProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_is_family_friendly_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::IS_FAMILY_FRIENDLY_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
