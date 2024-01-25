/// <https://schema.org/contributor>
pub const CONTRIBUTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/contributor";
/// <https://schema.org/contributor>
pub const CONTRIBUTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contributor";
/// <https://schema.org/contributor>
pub const CONTRIBUTOR_PROPERTY_LABEL: &str = "contributor";
pub struct ContributorPropertyIri;
impl PartialEq<&str> for ContributorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTRIBUTOR_PROPERTY_IRI_HTTP || *other == CONTRIBUTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContributorPropertyIri> for &str {
	fn eq(&self, other: &ContributorPropertyIri) -> bool {
		*self == CONTRIBUTOR_PROPERTY_IRI_HTTP || *self == CONTRIBUTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ContributorPropertyIriOrLabel;
impl PartialEq<&str> for ContributorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContributorPropertyIri || *other == CONTRIBUTOR_PROPERTY_LABEL
	}
}
impl PartialEq<ContributorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContributorPropertyIriOrLabel) -> bool {
		*self == ContributorPropertyIri || *self == CONTRIBUTOR_PROPERTY_LABEL
	}
}
