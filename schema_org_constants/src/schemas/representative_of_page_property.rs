/// <https://schema.org/representativeOfPage>
pub const REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/representativeOfPage";
/// <https://schema.org/representativeOfPage>
pub const REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/representativeOfPage";
/// <https://schema.org/representativeOfPage>
pub const REPRESENTATIVE_OF_PAGE_PROPERTY_LABEL: &str = "representativeOfPage";
pub struct RepresentativeOfPagePropertyIri;
impl PartialEq<&str> for RepresentativeOfPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTP
			|| *other == REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RepresentativeOfPagePropertyIri> for &str {
	fn eq(&self, other: &RepresentativeOfPagePropertyIri) -> bool {
		*self == REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTP
			|| *self == REPRESENTATIVE_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct RepresentativeOfPagePropertyIriOrLabel;
impl PartialEq<&str> for RepresentativeOfPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RepresentativeOfPagePropertyIri || *other == REPRESENTATIVE_OF_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<RepresentativeOfPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RepresentativeOfPagePropertyIriOrLabel) -> bool {
		*self == RepresentativeOfPagePropertyIri || *self == REPRESENTATIVE_OF_PAGE_PROPERTY_LABEL
	}
}
