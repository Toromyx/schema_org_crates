/// <https://schema.org/workPresented>
pub const WORK_PRESENTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/workPresented";
/// <https://schema.org/workPresented>
pub const WORK_PRESENTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workPresented";
/// <https://schema.org/workPresented>
pub const WORK_PRESENTED_PROPERTY_LABEL: &str = "workPresented";
pub struct WorkPresentedPropertyIri;
impl PartialEq<&str> for WorkPresentedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_PRESENTED_PROPERTY_IRI_HTTP || *other == WORK_PRESENTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkPresentedPropertyIri> for &str {
	fn eq(&self, other: &WorkPresentedPropertyIri) -> bool {
		*self == WORK_PRESENTED_PROPERTY_IRI_HTTP || *self == WORK_PRESENTED_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkPresentedPropertyIriOrLabel;
impl PartialEq<&str> for WorkPresentedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkPresentedPropertyIri || *other == WORK_PRESENTED_PROPERTY_LABEL
	}
}
impl PartialEq<WorkPresentedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkPresentedPropertyIriOrLabel) -> bool {
		*self == WorkPresentedPropertyIri || *self == WORK_PRESENTED_PROPERTY_LABEL
	}
}
