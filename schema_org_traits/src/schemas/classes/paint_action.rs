/// <https://schema.org/PaintAction>
pub trait FindPaintActionIds {
	type IdType;
	fn find_paint_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaintActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_paint_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PAINT_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PAINT_ACTION_IRI_HTTPS,
			})
		}
	}
}
