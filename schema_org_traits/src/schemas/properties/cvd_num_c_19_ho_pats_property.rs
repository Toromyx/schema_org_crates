/// <https://schema.org/cvdNumC19HOPats>
pub trait GetCvdNumC19HoPatsProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/cvdNumC19HOPats>
	fn get_cvd_num_c_19_ho_pats_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetCvdNumC19HoPatsProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_cvd_num_c_19_ho_pats_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::CVD_NUM_C_19_HO_PATS_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
