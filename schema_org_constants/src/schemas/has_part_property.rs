/// <https://schema.org/hasPart>
pub const HAS_PART_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasPart";
/// <https://schema.org/hasPart>
pub const HAS_PART_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasPart";
/// <https://schema.org/hasPart>
pub const HAS_PART_PROPERTY_LABEL: &str = "hasPart";
pub struct HasPartPropertyIri;
impl PartialEq<&str> for HasPartPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_PART_PROPERTY_IRI_HTTP || *other == HAS_PART_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasPartPropertyIri> for &str {
	fn eq(&self, other: &HasPartPropertyIri) -> bool {
		*self == HAS_PART_PROPERTY_IRI_HTTP || *self == HAS_PART_PROPERTY_IRI_HTTPS
	}
}
pub struct HasPartPropertyIriOrLabel;
impl PartialEq<&str> for HasPartPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasPartPropertyIri || *other == HAS_PART_PROPERTY_LABEL
	}
}
impl PartialEq<HasPartPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasPartPropertyIriOrLabel) -> bool {
		*self == HasPartPropertyIri || *self == HAS_PART_PROPERTY_LABEL
	}
}
