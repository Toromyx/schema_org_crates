/// <https://schema.org/AdultOrientedEnumeration>
pub const ADULT_ORIENTED_ENUMERATION_IRI_HTTP: &str = "http://schema.org/AdultOrientedEnumeration";
/// <https://schema.org/AdultOrientedEnumeration>
pub const ADULT_ORIENTED_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/AdultOrientedEnumeration";
/// <https://schema.org/AdultOrientedEnumeration>
pub const ADULT_ORIENTED_ENUMERATION_LABEL: &str = "AdultOrientedEnumeration";
pub struct AdultOrientedEnumerationIri;
impl PartialEq<&str> for AdultOrientedEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADULT_ORIENTED_ENUMERATION_IRI_HTTP
			|| *other == ADULT_ORIENTED_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<AdultOrientedEnumerationIri> for &str {
	fn eq(&self, other: &AdultOrientedEnumerationIri) -> bool {
		*self == ADULT_ORIENTED_ENUMERATION_IRI_HTTP
			|| *self == ADULT_ORIENTED_ENUMERATION_IRI_HTTPS
	}
}
pub struct AdultOrientedEnumerationIriOrLabel;
impl PartialEq<&str> for AdultOrientedEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdultOrientedEnumerationIri || *other == ADULT_ORIENTED_ENUMERATION_LABEL
	}
}
impl PartialEq<AdultOrientedEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &AdultOrientedEnumerationIriOrLabel) -> bool {
		*self == AdultOrientedEnumerationIri || *self == ADULT_ORIENTED_ENUMERATION_LABEL
	}
}
