/// <https://schema.org/hasRepresentation>
pub const HAS_REPRESENTATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasRepresentation";
/// <https://schema.org/hasRepresentation>
pub const HAS_REPRESENTATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasRepresentation";
/// <https://schema.org/hasRepresentation>
pub const HAS_REPRESENTATION_PROPERTY_LABEL: &str = "hasRepresentation";
pub struct HasRepresentationPropertyIri;
impl PartialEq<&str> for HasRepresentationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_REPRESENTATION_PROPERTY_IRI_HTTP
			|| *other == HAS_REPRESENTATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasRepresentationPropertyIri> for &str {
	fn eq(&self, other: &HasRepresentationPropertyIri) -> bool {
		*self == HAS_REPRESENTATION_PROPERTY_IRI_HTTP
			|| *self == HAS_REPRESENTATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasRepresentationPropertyIriOrLabel;
impl PartialEq<&str> for HasRepresentationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasRepresentationPropertyIri || *other == HAS_REPRESENTATION_PROPERTY_LABEL
	}
}
impl PartialEq<HasRepresentationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasRepresentationPropertyIriOrLabel) -> bool {
		*self == HasRepresentationPropertyIri || *self == HAS_REPRESENTATION_PROPERTY_LABEL
	}
}
