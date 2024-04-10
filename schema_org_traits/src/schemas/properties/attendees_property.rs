/// <https://schema.org/attendees>
#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
pub trait GetAttendeesProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/attendees>
	#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
	fn get_attendees_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetAttendeesProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_attendees_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::ATTENDEES_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => schema_org_constants::ATTENDEES_PROPERTY_IRI_HTTPS,
				},
			)
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetAttendeesProperty for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		type PropertyType = rdf_types_0_22::Object;
		fn get_attendees_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::ATTENDEES_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => schema_org_constants::ATTENDEES_PROPERTY_IRI_HTTPS,
				},
			)
		}
	}
}
