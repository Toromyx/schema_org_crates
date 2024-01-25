/// <https://schema.org/numberOfCredits>
pub const NUMBER_OF_CREDITS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfCredits";
/// <https://schema.org/numberOfCredits>
pub const NUMBER_OF_CREDITS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfCredits";
/// <https://schema.org/numberOfCredits>
pub const NUMBER_OF_CREDITS_PROPERTY_LABEL: &str = "numberOfCredits";
pub struct NumberOfCreditsPropertyIri;
impl PartialEq<&str> for NumberOfCreditsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_CREDITS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_CREDITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfCreditsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfCreditsPropertyIri) -> bool {
		*self == NUMBER_OF_CREDITS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_CREDITS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfCreditsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfCreditsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfCreditsPropertyIri || *other == NUMBER_OF_CREDITS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfCreditsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfCreditsPropertyIriOrLabel) -> bool {
		*self == NumberOfCreditsPropertyIri || *self == NUMBER_OF_CREDITS_PROPERTY_LABEL
	}
}
