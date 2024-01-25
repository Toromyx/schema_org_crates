/// <https://schema.org/hasHealthAspect>
pub const HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasHealthAspect";
/// <https://schema.org/hasHealthAspect>
pub const HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasHealthAspect";
/// <https://schema.org/hasHealthAspect>
pub const HAS_HEALTH_ASPECT_PROPERTY_LABEL: &str = "hasHealthAspect";
pub struct HasHealthAspectPropertyIri;
impl PartialEq<&str> for HasHealthAspectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTP
			|| *other == HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasHealthAspectPropertyIri> for &str {
	fn eq(&self, other: &HasHealthAspectPropertyIri) -> bool {
		*self == HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTP
			|| *self == HAS_HEALTH_ASPECT_PROPERTY_IRI_HTTPS
	}
}
pub struct HasHealthAspectPropertyIriOrLabel;
impl PartialEq<&str> for HasHealthAspectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasHealthAspectPropertyIri || *other == HAS_HEALTH_ASPECT_PROPERTY_LABEL
	}
}
impl PartialEq<HasHealthAspectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasHealthAspectPropertyIriOrLabel) -> bool {
		*self == HasHealthAspectPropertyIri || *self == HAS_HEALTH_ASPECT_PROPERTY_LABEL
	}
}
