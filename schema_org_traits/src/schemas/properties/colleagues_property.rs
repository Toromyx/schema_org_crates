/// <https://schema.org/colleagues>
#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
pub trait GetColleaguesProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/colleagues>
	#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
	fn get_colleagues_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetColleaguesProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_colleagues_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::COLLEAGUES_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => {
						schema_org_constants::COLLEAGUES_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
