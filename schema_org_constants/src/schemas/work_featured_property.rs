/// <https://schema.org/workFeatured>
pub const WORK_FEATURED_PROPERTY_IRI_HTTP: &str = "http://schema.org/workFeatured";
/// <https://schema.org/workFeatured>
pub const WORK_FEATURED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workFeatured";
/// <https://schema.org/workFeatured>
pub const WORK_FEATURED_PROPERTY_LABEL: &str = "workFeatured";
pub struct WorkFeaturedPropertyIri;
impl PartialEq<&str> for WorkFeaturedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_FEATURED_PROPERTY_IRI_HTTP || *other == WORK_FEATURED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkFeaturedPropertyIri> for &str {
	fn eq(&self, other: &WorkFeaturedPropertyIri) -> bool {
		*self == WORK_FEATURED_PROPERTY_IRI_HTTP || *self == WORK_FEATURED_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkFeaturedPropertyIriOrLabel;
impl PartialEq<&str> for WorkFeaturedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkFeaturedPropertyIri || *other == WORK_FEATURED_PROPERTY_LABEL
	}
}
impl PartialEq<WorkFeaturedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkFeaturedPropertyIriOrLabel) -> bool {
		*self == WorkFeaturedPropertyIri || *self == WORK_FEATURED_PROPERTY_LABEL
	}
}
