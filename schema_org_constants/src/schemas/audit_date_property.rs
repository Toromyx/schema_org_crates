/// <https://schema.org/auditDate>
pub const AUDIT_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/auditDate";
/// <https://schema.org/auditDate>
pub const AUDIT_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/auditDate";
/// <https://schema.org/auditDate>
pub const AUDIT_DATE_PROPERTY_LABEL: &str = "auditDate";
pub struct AuditDatePropertyIri;
impl PartialEq<&str> for AuditDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIT_DATE_PROPERTY_IRI_HTTP || *other == AUDIT_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AuditDatePropertyIri> for &str {
	fn eq(&self, other: &AuditDatePropertyIri) -> bool {
		*self == AUDIT_DATE_PROPERTY_IRI_HTTP || *self == AUDIT_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct AuditDatePropertyIriOrLabel;
impl PartialEq<&str> for AuditDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AuditDatePropertyIri || *other == AUDIT_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<AuditDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AuditDatePropertyIriOrLabel) -> bool {
		*self == AuditDatePropertyIri || *self == AUDIT_DATE_PROPERTY_LABEL
	}
}
