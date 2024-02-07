/// <https://schema.org/FDAnotEvaluated>
pub trait FindFdAnotEvaluatedIds {
	type IdType;
	/// <https://schema.org/FDAnotEvaluated>
	fn find_fd_anot_evaluated_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFdAnotEvaluatedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fd_anot_evaluated_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FD_ANOT_EVALUATED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FD_ANOT_EVALUATED_IRI_HTTPS,
			})
		}
	}
}
