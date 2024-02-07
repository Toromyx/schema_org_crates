/// <https://schema.org/ReimbursementCap>
pub trait FindReimbursementCapIds {
	type IdType;
	/// <https://schema.org/ReimbursementCap>
	fn find_reimbursement_cap_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReimbursementCapIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reimbursement_cap_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REIMBURSEMENT_CAP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REIMBURSEMENT_CAP_IRI_HTTPS,
			})
		}
	}
}
