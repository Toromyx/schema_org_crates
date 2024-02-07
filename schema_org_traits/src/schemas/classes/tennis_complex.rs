/// <https://schema.org/TennisComplex>
pub trait FindTennisComplexIds {
	type IdType;
	/// <https://schema.org/TennisComplex>
	fn find_tennis_complex_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTennisComplexIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_tennis_complex_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TENNIS_COMPLEX_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TENNIS_COMPLEX_IRI_HTTPS,
			})
		}
	}
}
