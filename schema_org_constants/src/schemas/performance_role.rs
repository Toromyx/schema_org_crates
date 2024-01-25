/// <https://schema.org/PerformanceRole>
pub const PERFORMANCE_ROLE_IRI_HTTP: &str = "http://schema.org/PerformanceRole";
/// <https://schema.org/PerformanceRole>
pub const PERFORMANCE_ROLE_IRI_HTTPS: &str = "https://schema.org/PerformanceRole";
/// <https://schema.org/PerformanceRole>
pub const PERFORMANCE_ROLE_LABEL: &str = "PerformanceRole";
pub struct PerformanceRoleIri;
impl PartialEq<&str> for PerformanceRoleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMANCE_ROLE_IRI_HTTP || *other == PERFORMANCE_ROLE_IRI_HTTPS
	}
}
impl PartialEq<PerformanceRoleIri> for &str {
	fn eq(&self, other: &PerformanceRoleIri) -> bool {
		*self == PERFORMANCE_ROLE_IRI_HTTP || *self == PERFORMANCE_ROLE_IRI_HTTPS
	}
}
pub struct PerformanceRoleIriOrLabel;
impl PartialEq<&str> for PerformanceRoleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformanceRoleIri || *other == PERFORMANCE_ROLE_LABEL
	}
}
impl PartialEq<PerformanceRoleIriOrLabel> for &str {
	fn eq(&self, other: &PerformanceRoleIriOrLabel) -> bool {
		*self == PerformanceRoleIri || *self == PERFORMANCE_ROLE_LABEL
	}
}
