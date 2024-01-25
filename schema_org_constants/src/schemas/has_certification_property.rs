/// <https://schema.org/hasCertification>
pub const HAS_CERTIFICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasCertification";
/// <https://schema.org/hasCertification>
pub const HAS_CERTIFICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasCertification";
/// <https://schema.org/hasCertification>
pub const HAS_CERTIFICATION_PROPERTY_LABEL: &str = "hasCertification";
pub struct HasCertificationPropertyIri;
impl PartialEq<&str> for HasCertificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_CERTIFICATION_PROPERTY_IRI_HTTP
			|| *other == HAS_CERTIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasCertificationPropertyIri> for &str {
	fn eq(&self, other: &HasCertificationPropertyIri) -> bool {
		*self == HAS_CERTIFICATION_PROPERTY_IRI_HTTP
			|| *self == HAS_CERTIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasCertificationPropertyIriOrLabel;
impl PartialEq<&str> for HasCertificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasCertificationPropertyIri || *other == HAS_CERTIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<HasCertificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasCertificationPropertyIriOrLabel) -> bool {
		*self == HasCertificationPropertyIri || *self == HAS_CERTIFICATION_PROPERTY_LABEL
	}
}
