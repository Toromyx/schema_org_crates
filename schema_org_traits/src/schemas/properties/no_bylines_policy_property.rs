/// <https://schema.org/noBylinesPolicy>
pub trait GetNoBylinesPolicyProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/noBylinesPolicy>
	fn get_no_bylines_policy_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetNoBylinesPolicyProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_no_bylines_policy_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::NO_BYLINES_POLICY_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::NO_BYLINES_POLICY_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
