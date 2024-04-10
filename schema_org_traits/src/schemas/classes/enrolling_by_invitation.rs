/// <https://schema.org/EnrollingByInvitation>
pub trait FindEnrollingByInvitationIds {
	type IdType;
	/// <https://schema.org/EnrollingByInvitation>
	fn find_enrolling_by_invitation_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEnrollingByInvitationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_enrolling_by_invitation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENROLLING_BY_INVITATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::ENROLLING_BY_INVITATION_IRI_HTTPS
				}
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEnrollingByInvitationIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_enrolling_by_invitation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENROLLING_BY_INVITATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::ENROLLING_BY_INVITATION_IRI_HTTPS
				}
			})
		}
	}
}
