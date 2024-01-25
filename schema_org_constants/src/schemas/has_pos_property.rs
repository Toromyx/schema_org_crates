/// <https://schema.org/hasPOS>
pub const HAS_POS_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasPOS";
/// <https://schema.org/hasPOS>
pub const HAS_POS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasPOS";
/// <https://schema.org/hasPOS>
pub const HAS_POS_PROPERTY_LABEL: &str = "hasPOS";
pub struct HasPosPropertyIri;
impl PartialEq<&str> for HasPosPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_POS_PROPERTY_IRI_HTTP || *other == HAS_POS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasPosPropertyIri> for &str {
	fn eq(&self, other: &HasPosPropertyIri) -> bool {
		*self == HAS_POS_PROPERTY_IRI_HTTP || *self == HAS_POS_PROPERTY_IRI_HTTPS
	}
}
pub struct HasPosPropertyIriOrLabel;
impl PartialEq<&str> for HasPosPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasPosPropertyIri || *other == HAS_POS_PROPERTY_LABEL
	}
}
impl PartialEq<HasPosPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasPosPropertyIriOrLabel) -> bool {
		*self == HasPosPropertyIri || *self == HAS_POS_PROPERTY_LABEL
	}
}
