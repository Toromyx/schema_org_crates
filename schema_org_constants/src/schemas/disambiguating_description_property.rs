/// <https://schema.org/disambiguatingDescription>
pub const DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/disambiguatingDescription";
/// <https://schema.org/disambiguatingDescription>
pub const DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/disambiguatingDescription";
/// <https://schema.org/disambiguatingDescription>
pub const DISAMBIGUATING_DESCRIPTION_PROPERTY_LABEL: &str = "disambiguatingDescription";
pub struct DisambiguatingDescriptionPropertyIri;
impl PartialEq<&str> for DisambiguatingDescriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *other == DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DisambiguatingDescriptionPropertyIri> for &str {
	fn eq(&self, other: &DisambiguatingDescriptionPropertyIri) -> bool {
		*self == DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *self == DISAMBIGUATING_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct DisambiguatingDescriptionPropertyIriOrLabel;
impl PartialEq<&str> for DisambiguatingDescriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DisambiguatingDescriptionPropertyIri
			|| *other == DISAMBIGUATING_DESCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<DisambiguatingDescriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DisambiguatingDescriptionPropertyIriOrLabel) -> bool {
		*self == DisambiguatingDescriptionPropertyIri
			|| *self == DISAMBIGUATING_DESCRIPTION_PROPERTY_LABEL
	}
}
