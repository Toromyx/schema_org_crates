use super::*;
/// <https://schema.org/JobPosting>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JobPosting {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "applicant-location-requirements-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "applicantLocationRequirements"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/applicantLocationRequirements")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/applicantLocationRequirements")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#applicant_location_requirements: Vec<ApplicantLocationRequirementsProperty>,
    #[cfg(any(
        any(
            feature = "application-contact-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "applicationContact"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/applicationContact")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/applicationContact")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#application_contact: Vec<ApplicationContactProperty>,
    #[cfg(any(
        any(
            feature = "base-salary-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "baseSalary"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/baseSalary"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/baseSalary"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#base_salary: Vec<BaseSalaryProperty>,
    #[cfg(any(
        any(
            feature = "benefits-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "benefits"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/benefits"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/benefits"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#benefits: Vec<BenefitsProperty>,
    #[cfg(any(
        any(
            feature = "date-posted-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "datePosted"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/datePosted"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/datePosted"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#date_posted: Vec<DatePostedProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "direct-apply-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "directApply"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/directApply"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/directApply"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#direct_apply: Vec<DirectApplyProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "education-requirements-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationRequirements"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/educationRequirements")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/educationRequirements")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#education_requirements: Vec<EducationRequirementsProperty>,
    #[cfg(any(
        any(
            feature = "eligibility-to-work-requirement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "eligibilityToWorkRequirement"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/eligibilityToWorkRequirement")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/eligibilityToWorkRequirement")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#eligibility_to_work_requirement: Vec<EligibilityToWorkRequirementProperty>,
    #[cfg(any(
        any(
            feature = "employer-overview-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "employerOverview"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/employerOverview")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/employerOverview"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#employer_overview: Vec<EmployerOverviewProperty>,
    #[cfg(any(
        any(
            feature = "employment-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "employmentType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/employmentType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/employmentType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#employment_type: Vec<EmploymentTypeProperty>,
    #[cfg(any(
        any(
            feature = "employment-unit-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "employmentUnit"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/employmentUnit"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/employmentUnit"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#employment_unit: Vec<EmploymentUnitProperty>,
    #[cfg(any(
        any(
            feature = "estimated-salary-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "estimatedSalary"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/estimatedSalary"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/estimatedSalary"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#estimated_salary: Vec<EstimatedSalaryProperty>,
    #[cfg(any(
        any(
            feature = "experience-in-place-of-education-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "experienceInPlaceOfEducation"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/experienceInPlaceOfEducation")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/experienceInPlaceOfEducation")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#experience_in_place_of_education: Vec<ExperienceInPlaceOfEducationProperty>,
    #[cfg(any(
        any(
            feature = "experience-requirements-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "experienceRequirements"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/experienceRequirements")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/experienceRequirements")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#experience_requirements: Vec<ExperienceRequirementsProperty>,
    #[cfg(any(
        any(
            feature = "hiring-organization-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hiringOrganization"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hiringOrganization")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hiringOrganization")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#hiring_organization: Vec<HiringOrganizationProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "incentive-compensation-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "incentiveCompensation"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/incentiveCompensation")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/incentiveCompensation")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#incentive_compensation: Vec<IncentiveCompensationProperty>,
    #[cfg(any(
        any(
            feature = "incentives-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "incentives"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/incentives"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/incentives"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#incentives: Vec<IncentivesProperty>,
    #[cfg(any(
        any(
            feature = "industry-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "industry"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/industry"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/industry"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#industry: Vec<IndustryProperty>,
    #[cfg(any(
        any(
            feature = "job-benefits-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jobBenefits"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/jobBenefits"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/jobBenefits"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#job_benefits: Vec<JobBenefitsProperty>,
    #[cfg(any(
        any(
            feature = "job-immediate-start-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jobImmediateStart"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/jobImmediateStart")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/jobImmediateStart")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#job_immediate_start: Vec<JobImmediateStartProperty>,
    #[cfg(any(
        any(
            feature = "job-location-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jobLocation"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/jobLocation"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/jobLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#job_location: Vec<JobLocationProperty>,
    #[cfg(any(
        any(
            feature = "job-location-type-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jobLocationType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/jobLocationType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/jobLocationType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#job_location_type: Vec<JobLocationTypeProperty>,
    #[cfg(any(
        any(
            feature = "job-start-date-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jobStartDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/jobStartDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/jobStartDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#job_start_date: Vec<JobStartDateProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/mainEntityOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "occupational-category-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "occupationalCategory"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/occupationalCategory")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/occupationalCategory")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#occupational_category: Vec<OccupationalCategoryProperty>,
    #[cfg(any(
        any(
            feature = "physical-requirement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "physicalRequirement"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/physicalRequirement")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/physicalRequirement")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#physical_requirement: Vec<PhysicalRequirementProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "qualifications-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "qualifications"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/qualifications"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/qualifications"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#qualifications: Vec<QualificationsProperty>,
    #[cfg(any(
        any(
            feature = "relevant-occupation-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "relevantOccupation"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/relevantOccupation")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/relevantOccupation")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#relevant_occupation: Vec<RelevantOccupationProperty>,
    #[cfg(any(
        any(
            feature = "responsibilities-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "responsibilities"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/responsibilities")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/responsibilities"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#responsibilities: Vec<ResponsibilitiesProperty>,
    #[cfg(any(
        any(
            feature = "salary-currency-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "salaryCurrency"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/salaryCurrency"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/salaryCurrency"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#salary_currency: Vec<SalaryCurrencyProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "security-clearance-requirement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "securityClearanceRequirement"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/securityClearanceRequirement")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/securityClearanceRequirement")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#security_clearance_requirement: Vec<SecurityClearanceRequirementProperty>,
    #[cfg(any(
        any(
            feature = "sensory-requirement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sensoryRequirement"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/sensoryRequirement")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/sensoryRequirement")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sensory_requirement: Vec<SensoryRequirementProperty>,
    #[cfg(any(
        any(feature = "skills-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "skills"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/skills"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/skills"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#skills: Vec<SkillsProperty>,
    #[cfg(any(
        any(
            feature = "special-commitments-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "specialCommitments"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/specialCommitments")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/specialCommitments")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#special_commitments: Vec<SpecialCommitmentsProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "title-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "title"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/title"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/title"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#title: Vec<TitleProperty>,
    #[cfg(any(
        any(
            feature = "total-job-openings-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "totalJobOpenings"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/totalJobOpenings")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/totalJobOpenings"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#total_job_openings: Vec<TotalJobOpeningsProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "valid-through-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "validThrough"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/validThrough"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/validThrough"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#valid_through: Vec<ValidThroughProperty>,
    #[cfg(any(
        any(
            feature = "work-hours-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "workHours"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/workHours"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/workHours"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#work_hours: Vec<WorkHoursProperty>,
}
