/// <https://schema.org/numberOfItems>
pub const NUMBER_OF_ITEMS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfItems";
/// <https://schema.org/numberOfItems>
pub const NUMBER_OF_ITEMS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfItems";
/// <https://schema.org/numberOfItems>
pub const NUMBER_OF_ITEMS_PROPERTY_LABEL: &str = "numberOfItems";
pub struct NumberOfItemsPropertyIri;
impl PartialEq<&str> for NumberOfItemsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_ITEMS_PROPERTY_IRI_HTTP || *other == NUMBER_OF_ITEMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfItemsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfItemsPropertyIri) -> bool {
		*self == NUMBER_OF_ITEMS_PROPERTY_IRI_HTTP || *self == NUMBER_OF_ITEMS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfItemsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfItemsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfItemsPropertyIri || *other == NUMBER_OF_ITEMS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfItemsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfItemsPropertyIriOrLabel) -> bool {
		*self == NumberOfItemsPropertyIri || *self == NUMBER_OF_ITEMS_PROPERTY_LABEL
	}
}
