/// <https://schema.org/originatesFrom>
pub const ORIGINATES_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/originatesFrom";
/// <https://schema.org/originatesFrom>
pub const ORIGINATES_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/originatesFrom";
/// <https://schema.org/originatesFrom>
pub const ORIGINATES_FROM_PROPERTY_LABEL: &str = "originatesFrom";
pub struct OriginatesFromPropertyIri;
impl PartialEq<&str> for OriginatesFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGINATES_FROM_PROPERTY_IRI_HTTP || *other == ORIGINATES_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OriginatesFromPropertyIri> for &str {
	fn eq(&self, other: &OriginatesFromPropertyIri) -> bool {
		*self == ORIGINATES_FROM_PROPERTY_IRI_HTTP || *self == ORIGINATES_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct OriginatesFromPropertyIriOrLabel;
impl PartialEq<&str> for OriginatesFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginatesFromPropertyIri || *other == ORIGINATES_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<OriginatesFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OriginatesFromPropertyIriOrLabel) -> bool {
		*self == OriginatesFromPropertyIri || *self == ORIGINATES_FROM_PROPERTY_LABEL
	}
}
