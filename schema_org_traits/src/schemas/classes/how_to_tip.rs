/// <https://schema.org/HowToTip>
pub trait FindHowToTipIds {
	type IdType;
	fn find_how_to_tip_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHowToTipIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_how_to_tip_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HOW_TO_TIP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HOW_TO_TIP_IRI_HTTPS,
			})
		}
	}
}
