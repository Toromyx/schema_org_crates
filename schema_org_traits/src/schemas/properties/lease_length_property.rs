/// <https://schema.org/leaseLength>
pub trait GetLeaseLengthProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/leaseLength>
	fn get_lease_length_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetLeaseLengthProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_lease_length_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::LEASE_LENGTH_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::LEASE_LENGTH_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
