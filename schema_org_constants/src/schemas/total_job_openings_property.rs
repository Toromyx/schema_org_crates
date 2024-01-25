/// <https://schema.org/totalJobOpenings>
pub const TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTP: &str = "http://schema.org/totalJobOpenings";
/// <https://schema.org/totalJobOpenings>
pub const TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/totalJobOpenings";
/// <https://schema.org/totalJobOpenings>
pub const TOTAL_JOB_OPENINGS_PROPERTY_LABEL: &str = "totalJobOpenings";
pub struct TotalJobOpeningsPropertyIri;
impl PartialEq<&str> for TotalJobOpeningsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTP
			|| *other == TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TotalJobOpeningsPropertyIri> for &str {
	fn eq(&self, other: &TotalJobOpeningsPropertyIri) -> bool {
		*self == TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTP
			|| *self == TOTAL_JOB_OPENINGS_PROPERTY_IRI_HTTPS
	}
}
pub struct TotalJobOpeningsPropertyIriOrLabel;
impl PartialEq<&str> for TotalJobOpeningsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TotalJobOpeningsPropertyIri || *other == TOTAL_JOB_OPENINGS_PROPERTY_LABEL
	}
}
impl PartialEq<TotalJobOpeningsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TotalJobOpeningsPropertyIriOrLabel) -> bool {
		*self == TotalJobOpeningsPropertyIri || *self == TOTAL_JOB_OPENINGS_PROPERTY_LABEL
	}
}
