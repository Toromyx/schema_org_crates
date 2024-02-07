/// <https://schema.org/stupidProperty>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub trait GetStupidPropertyProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/stupidProperty>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
	fn get_stupid_property_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetStupidPropertyProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_stupid_property_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::STUPID_PROPERTY_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::STUPID_PROPERTY_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
