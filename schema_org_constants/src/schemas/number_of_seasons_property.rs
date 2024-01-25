/// <https://schema.org/numberOfSeasons>
pub const NUMBER_OF_SEASONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfSeasons";
/// <https://schema.org/numberOfSeasons>
pub const NUMBER_OF_SEASONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfSeasons";
/// <https://schema.org/numberOfSeasons>
pub const NUMBER_OF_SEASONS_PROPERTY_LABEL: &str = "numberOfSeasons";
pub struct NumberOfSeasonsPropertyIri;
impl PartialEq<&str> for NumberOfSeasonsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_SEASONS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_SEASONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfSeasonsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfSeasonsPropertyIri) -> bool {
		*self == NUMBER_OF_SEASONS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_SEASONS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfSeasonsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfSeasonsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfSeasonsPropertyIri || *other == NUMBER_OF_SEASONS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfSeasonsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfSeasonsPropertyIriOrLabel) -> bool {
		*self == NumberOfSeasonsPropertyIri || *self == NUMBER_OF_SEASONS_PROPERTY_LABEL
	}
}
