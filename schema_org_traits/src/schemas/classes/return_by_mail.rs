/// <https://schema.org/ReturnByMail>
pub trait FindReturnByMailIds {
	type IdType;
	/// <https://schema.org/ReturnByMail>
	fn find_return_by_mail_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnByMailIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_by_mail_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_BY_MAIL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RETURN_BY_MAIL_IRI_HTTPS,
			})
		}
	}
}
