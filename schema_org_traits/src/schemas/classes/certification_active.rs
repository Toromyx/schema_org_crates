/// <https://schema.org/CertificationActive>
pub trait FindCertificationActiveIds {
	type IdType;
	/// <https://schema.org/CertificationActive>
	fn find_certification_active_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCertificationActiveIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_certification_active_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CERTIFICATION_ACTIVE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CERTIFICATION_ACTIVE_IRI_HTTPS,
			})
		}
	}
}
