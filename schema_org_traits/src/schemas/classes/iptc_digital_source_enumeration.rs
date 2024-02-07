/// <https://schema.org/IPTCDigitalSourceEnumeration>
pub trait FindIptcDigitalSourceEnumerationIds {
	type IdType;
	/// <https://schema.org/IPTCDigitalSourceEnumeration>
	fn find_iptc_digital_source_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIptcDigitalSourceEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_iptc_digital_source_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
