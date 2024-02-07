/// <https://schema.org/isicV4>
pub trait GetIsicV4Property {
	type IdType;
	type PropertyType;
	/// <https://schema.org/isicV4>
	fn get_isic_v_4_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetIsicV4Property for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_isic_v_4_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::ISIC_V_4_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => schema_org_constants::ISIC_V_4_PROPERTY_IRI_HTTPS,
				},
			)
		}
	}
}
