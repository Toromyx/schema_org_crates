/// <https://schema.org/UsageOrScheduleHealthAspect>
pub const USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/UsageOrScheduleHealthAspect";
/// <https://schema.org/UsageOrScheduleHealthAspect>
pub const USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/UsageOrScheduleHealthAspect";
/// <https://schema.org/UsageOrScheduleHealthAspect>
pub const USAGE_OR_SCHEDULE_HEALTH_ASPECT_LABEL: &str = "UsageOrScheduleHealthAspect";
pub struct UsageOrScheduleHealthAspectIri;
impl PartialEq<&str> for UsageOrScheduleHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTP
			|| *other == USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<UsageOrScheduleHealthAspectIri> for &str {
	fn eq(&self, other: &UsageOrScheduleHealthAspectIri) -> bool {
		*self == USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTP
			|| *self == USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct UsageOrScheduleHealthAspectIriOrLabel;
impl PartialEq<&str> for UsageOrScheduleHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsageOrScheduleHealthAspectIri || *other == USAGE_OR_SCHEDULE_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<UsageOrScheduleHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &UsageOrScheduleHealthAspectIriOrLabel) -> bool {
		*self == UsageOrScheduleHealthAspectIri || *self == USAGE_OR_SCHEDULE_HEALTH_ASPECT_LABEL
	}
}
