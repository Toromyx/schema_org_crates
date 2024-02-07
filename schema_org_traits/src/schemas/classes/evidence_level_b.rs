/// <https://schema.org/EvidenceLevelB>
pub trait FindEvidenceLevelBIds {
	type IdType;
	/// <https://schema.org/EvidenceLevelB>
	fn find_evidence_level_b_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEvidenceLevelBIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_evidence_level_b_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EVIDENCE_LEVEL_B_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EVIDENCE_LEVEL_B_IRI_HTTPS,
			})
		}
	}
}
