/// <https://schema.org/WeaponConsideration>
pub trait FindWeaponConsiderationIds {
	type IdType;
	/// <https://schema.org/WeaponConsideration>
	fn find_weapon_consideration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWeaponConsiderationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_weapon_consideration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEAPON_CONSIDERATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WEAPON_CONSIDERATION_IRI_HTTPS,
			})
		}
	}
}
