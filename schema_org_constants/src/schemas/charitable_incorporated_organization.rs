/// <https://schema.org/CharitableIncorporatedOrganization>
pub const CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTP: &str =
	"http://schema.org/CharitableIncorporatedOrganization";
/// <https://schema.org/CharitableIncorporatedOrganization>
pub const CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTPS: &str =
	"https://schema.org/CharitableIncorporatedOrganization";
/// <https://schema.org/CharitableIncorporatedOrganization>
pub const CHARITABLE_INCORPORATED_ORGANIZATION_LABEL: &str = "CharitableIncorporatedOrganization";
pub struct CharitableIncorporatedOrganizationIri;
impl PartialEq<&str> for CharitableIncorporatedOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTP
			|| *other == CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<CharitableIncorporatedOrganizationIri> for &str {
	fn eq(&self, other: &CharitableIncorporatedOrganizationIri) -> bool {
		*self == CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTP
			|| *self == CHARITABLE_INCORPORATED_ORGANIZATION_IRI_HTTPS
	}
}
pub struct CharitableIncorporatedOrganizationIriOrLabel;
impl PartialEq<&str> for CharitableIncorporatedOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CharitableIncorporatedOrganizationIri
			|| *other == CHARITABLE_INCORPORATED_ORGANIZATION_LABEL
	}
}
impl PartialEq<CharitableIncorporatedOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &CharitableIncorporatedOrganizationIriOrLabel) -> bool {
		*self == CharitableIncorporatedOrganizationIri
			|| *self == CHARITABLE_INCORPORATED_ORGANIZATION_LABEL
	}
}
