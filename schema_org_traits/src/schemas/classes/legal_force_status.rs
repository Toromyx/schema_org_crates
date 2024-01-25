/// <https://schema.org/LegalForceStatus>
pub trait FindLegalForceStatusIds {
	type IdType;
	fn find_legal_force_status_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLegalForceStatusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_legal_force_status_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LEGAL_FORCE_STATUS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LEGAL_FORCE_STATUS_IRI_HTTPS,
			})
		}
	}
}
