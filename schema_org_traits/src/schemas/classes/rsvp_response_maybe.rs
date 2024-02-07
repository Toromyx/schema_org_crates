/// <https://schema.org/RsvpResponseMaybe>
pub trait FindRsvpResponseMaybeIds {
	type IdType;
	/// <https://schema.org/RsvpResponseMaybe>
	fn find_rsvp_response_maybe_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRsvpResponseMaybeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_rsvp_response_maybe_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RSVP_RESPONSE_MAYBE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RSVP_RESPONSE_MAYBE_IRI_HTTPS,
			})
		}
	}
}
