/// <https://schema.org/targetUrl>
pub trait GetTargetUrlProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/targetUrl>
	fn get_target_url_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetTargetUrlProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_target_url_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::TARGET_URL_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => {
						schema_org_constants::TARGET_URL_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
