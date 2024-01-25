/// <https://schema.org/hasMap>
pub const HAS_MAP_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMap";
/// <https://schema.org/hasMap>
pub const HAS_MAP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasMap";
/// <https://schema.org/hasMap>
pub const HAS_MAP_PROPERTY_LABEL: &str = "hasMap";
pub struct HasMapPropertyIri;
impl PartialEq<&str> for HasMapPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MAP_PROPERTY_IRI_HTTP || *other == HAS_MAP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMapPropertyIri> for &str {
	fn eq(&self, other: &HasMapPropertyIri) -> bool {
		*self == HAS_MAP_PROPERTY_IRI_HTTP || *self == HAS_MAP_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMapPropertyIriOrLabel;
impl PartialEq<&str> for HasMapPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMapPropertyIri || *other == HAS_MAP_PROPERTY_LABEL
	}
}
impl PartialEq<HasMapPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMapPropertyIriOrLabel) -> bool {
		*self == HasMapPropertyIri || *self == HAS_MAP_PROPERTY_LABEL
	}
}
