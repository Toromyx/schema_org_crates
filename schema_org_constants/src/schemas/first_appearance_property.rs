/// <https://schema.org/firstAppearance>
pub const FIRST_APPEARANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/firstAppearance";
/// <https://schema.org/firstAppearance>
pub const FIRST_APPEARANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/firstAppearance";
/// <https://schema.org/firstAppearance>
pub const FIRST_APPEARANCE_PROPERTY_LABEL: &str = "firstAppearance";
pub struct FirstAppearancePropertyIri;
impl PartialEq<&str> for FirstAppearancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FIRST_APPEARANCE_PROPERTY_IRI_HTTP
			|| *other == FIRST_APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FirstAppearancePropertyIri> for &str {
	fn eq(&self, other: &FirstAppearancePropertyIri) -> bool {
		*self == FIRST_APPEARANCE_PROPERTY_IRI_HTTP || *self == FIRST_APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct FirstAppearancePropertyIriOrLabel;
impl PartialEq<&str> for FirstAppearancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FirstAppearancePropertyIri || *other == FIRST_APPEARANCE_PROPERTY_LABEL
	}
}
impl PartialEq<FirstAppearancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FirstAppearancePropertyIriOrLabel) -> bool {
		*self == FirstAppearancePropertyIri || *self == FIRST_APPEARANCE_PROPERTY_LABEL
	}
}
