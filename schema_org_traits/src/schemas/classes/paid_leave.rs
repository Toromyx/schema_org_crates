/// <https://schema.org/PaidLeave>
pub trait FindPaidLeaveIds {
	type IdType;
	/// <https://schema.org/PaidLeave>
	fn find_paid_leave_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaidLeaveIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_paid_leave_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PAID_LEAVE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PAID_LEAVE_IRI_HTTPS,
			})
		}
	}
}
