/// <https://schema.org/iso6523Code>
pub trait GetIso6523CodeProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/iso6523Code>
	fn get_iso_6523_code_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetIso6523CodeProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_iso_6523_code_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::ISO_6523_CODE_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::ISO_6523_CODE_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
