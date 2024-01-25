/// <https://schema.org/amountOfThisGood>
pub const AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/amountOfThisGood";
/// <https://schema.org/amountOfThisGood>
pub const AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/amountOfThisGood";
/// <https://schema.org/amountOfThisGood>
pub const AMOUNT_OF_THIS_GOOD_PROPERTY_LABEL: &str = "amountOfThisGood";
pub struct AmountOfThisGoodPropertyIri;
impl PartialEq<&str> for AmountOfThisGoodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTP
			|| *other == AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AmountOfThisGoodPropertyIri> for &str {
	fn eq(&self, other: &AmountOfThisGoodPropertyIri) -> bool {
		*self == AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTP
			|| *self == AMOUNT_OF_THIS_GOOD_PROPERTY_IRI_HTTPS
	}
}
pub struct AmountOfThisGoodPropertyIriOrLabel;
impl PartialEq<&str> for AmountOfThisGoodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmountOfThisGoodPropertyIri || *other == AMOUNT_OF_THIS_GOOD_PROPERTY_LABEL
	}
}
impl PartialEq<AmountOfThisGoodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AmountOfThisGoodPropertyIriOrLabel) -> bool {
		*self == AmountOfThisGoodPropertyIri || *self == AMOUNT_OF_THIS_GOOD_PROPERTY_LABEL
	}
}
