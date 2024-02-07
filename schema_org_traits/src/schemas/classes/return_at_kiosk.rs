/// <https://schema.org/ReturnAtKiosk>
pub trait FindReturnAtKioskIds {
	type IdType;
	/// <https://schema.org/ReturnAtKiosk>
	fn find_return_at_kiosk_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnAtKioskIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_at_kiosk_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_AT_KIOSK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RETURN_AT_KIOSK_IRI_HTTPS,
			})
		}
	}
}
