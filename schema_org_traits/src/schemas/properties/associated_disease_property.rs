/// <https://schema.org/associatedDisease>
pub trait GetAssociatedDiseaseProperty {
	type IdType;
	type PropertyType;
	fn get_associated_disease_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetAssociatedDiseaseProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_associated_disease_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::ASSOCIATED_DISEASE_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::ASSOCIATED_DISEASE_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
