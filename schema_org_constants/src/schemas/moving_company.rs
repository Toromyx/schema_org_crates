/// <https://schema.org/MovingCompany>
pub const MOVING_COMPANY_IRI_HTTP: &str = "http://schema.org/MovingCompany";
/// <https://schema.org/MovingCompany>
pub const MOVING_COMPANY_IRI_HTTPS: &str = "https://schema.org/MovingCompany";
/// <https://schema.org/MovingCompany>
pub const MOVING_COMPANY_LABEL: &str = "MovingCompany";
pub struct MovingCompanyIri;
impl PartialEq<&str> for MovingCompanyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOVING_COMPANY_IRI_HTTP || *other == MOVING_COMPANY_IRI_HTTPS
	}
}
impl PartialEq<MovingCompanyIri> for &str {
	fn eq(&self, other: &MovingCompanyIri) -> bool {
		*self == MOVING_COMPANY_IRI_HTTP || *self == MOVING_COMPANY_IRI_HTTPS
	}
}
pub struct MovingCompanyIriOrLabel;
impl PartialEq<&str> for MovingCompanyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MovingCompanyIri || *other == MOVING_COMPANY_LABEL
	}
}
impl PartialEq<MovingCompanyIriOrLabel> for &str {
	fn eq(&self, other: &MovingCompanyIriOrLabel) -> bool {
		*self == MovingCompanyIri || *self == MOVING_COMPANY_LABEL
	}
}
