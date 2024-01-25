/// <https://schema.org/hasAdultConsideration>
pub const HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasAdultConsideration";
/// <https://schema.org/hasAdultConsideration>
pub const HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasAdultConsideration";
/// <https://schema.org/hasAdultConsideration>
pub const HAS_ADULT_CONSIDERATION_PROPERTY_LABEL: &str = "hasAdultConsideration";
pub struct HasAdultConsiderationPropertyIri;
impl PartialEq<&str> for HasAdultConsiderationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTP
			|| *other == HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasAdultConsiderationPropertyIri> for &str {
	fn eq(&self, other: &HasAdultConsiderationPropertyIri) -> bool {
		*self == HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTP
			|| *self == HAS_ADULT_CONSIDERATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasAdultConsiderationPropertyIriOrLabel;
impl PartialEq<&str> for HasAdultConsiderationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasAdultConsiderationPropertyIri
			|| *other == HAS_ADULT_CONSIDERATION_PROPERTY_LABEL
	}
}
impl PartialEq<HasAdultConsiderationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasAdultConsiderationPropertyIriOrLabel) -> bool {
		*self == HasAdultConsiderationPropertyIri || *self == HAS_ADULT_CONSIDERATION_PROPERTY_LABEL
	}
}
