/// <https://schema.org/employerOverview>
pub const EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTP: &str = "http://schema.org/employerOverview";
/// <https://schema.org/employerOverview>
pub const EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTPS: &str = "https://schema.org/employerOverview";
/// <https://schema.org/employerOverview>
pub const EMPLOYER_OVERVIEW_PROPERTY_LABEL: &str = "employerOverview";
pub struct EmployerOverviewPropertyIri;
impl PartialEq<&str> for EmployerOverviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTP
			|| *other == EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmployerOverviewPropertyIri> for &str {
	fn eq(&self, other: &EmployerOverviewPropertyIri) -> bool {
		*self == EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTP
			|| *self == EMPLOYER_OVERVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct EmployerOverviewPropertyIriOrLabel;
impl PartialEq<&str> for EmployerOverviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployerOverviewPropertyIri || *other == EMPLOYER_OVERVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<EmployerOverviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmployerOverviewPropertyIriOrLabel) -> bool {
		*self == EmployerOverviewPropertyIri || *self == EMPLOYER_OVERVIEW_PROPERTY_LABEL
	}
}
