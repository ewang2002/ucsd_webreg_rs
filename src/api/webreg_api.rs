#![cfg(feature = "api")]

use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use webweg::wrapper::{CourseLevelFilter, DayOfWeek, SearchRequestBuilder, SearchType};

use crate::api::util::{api_get_general, process_return};
use crate::types::WrapperState;

#[derive(Deserialize)]
pub struct CourseQueryStr {
    subject: String,
    number: String,
}

/// An endpoint for getting the course information for a particular term.
///
/// # Usage
/// The endpoint should be called like so:
/// ```
/// /<term>?subject=CSE&number=8B
/// ```
pub async fn api_get_course_info(
    Path(term): Path<String>,
    Query(crsc): Query<CourseQueryStr>,
    State(s): State<WrapperState>,
) -> Response {
    api_get_general(
        term.as_str(),
        move |term_info| async move {
            let guard = term_info.general_wrapper.lock().await;
            process_return(guard.get_course_info(&crsc.subject, &crsc.number).await)
        },
        s,
    )
    .await
}

/// An endpoint for getting the course prerequisites for a particular term.
///
/// # Usage
/// The endpoint should be called like so:
/// ```
/// /<term>?subject=CSE&number=8B
/// ```
pub async fn api_get_prereqs(
    Path(term): Path<String>,
    Query(crsc): Query<CourseQueryStr>,
    State(s): State<WrapperState>,
) -> Response {
    api_get_general(
        term.as_str(),
        move |term_info| async move {
            let guard = term_info.general_wrapper.lock().await;
            process_return(guard.get_prereqs(&crsc.subject, &crsc.number).await)
        },
        s,
    )
    .await
}

#[derive(Deserialize)]
pub struct CourseSearchJsonBody {
    subjects: Option<Vec<String>>,
    courses: Option<Vec<String>>,
    departments: Option<Vec<String>>,
    instructor: Option<String>,
    title: Option<String>,
    only_allow_open: Option<bool>,
    show_lower_div: Option<bool>,
    show_upper_div: Option<bool>,
    show_grad_div: Option<bool>,
    start_min: Option<i32>,
    start_hr: Option<i32>,
    end_min: Option<i32>,
    end_hr: Option<i32>,
    days: Option<Vec<String>>,
}

/// An endpoint for searching for courses for a particular term.
///
/// # Usage
/// The endpoint should be called like so:
/// ```
/// /<term>
/// ```
/// with the body being a JSON with the relevant search information.
#[axum_macros::debug_handler]
pub async fn api_get_search_courses(
    Path(term): Path<String>,
    State(s): State<WrapperState>,
    // The Json needs to be the last parameter since its request body is being consumed.
    Json(search_info): Json<CourseSearchJsonBody>,
) -> Response {
    api_get_general(
        term.as_str(),
        move |term_info| async move {
            let mut query_builder = SearchRequestBuilder::new();

            // Add the subject
            if let Some(all_subjects) = search_info.subjects {
                for subj in all_subjects {
                    query_builder = query_builder.add_subject(subj);
                }
            }

            // Add the courses
            if let Some(all_courses) = search_info.courses {
                for crsc in all_courses {
                    query_builder = query_builder.add_course(crsc);
                }
            }

            // Add the departments
            if let Some(all_departments) = search_info.departments {
                for dept in all_departments {
                    query_builder = query_builder.add_department(dept);
                }
            }

            // Add the instructor
            if let Some(instructor) = search_info.instructor {
                query_builder = query_builder.set_instructor(instructor);
            }

            // Add the title
            if let Some(title) = search_info.title {
                query_builder = query_builder.set_title(title);
            }

            if let Some(only_allow_open) = search_info.only_allow_open {
                if only_allow_open {
                    query_builder = query_builder.only_allow_open();
                }
            }

            if let Some(show_lower) = search_info.show_lower_div {
                if show_lower {
                    query_builder =
                        query_builder.filter_courses_by(CourseLevelFilter::LowerDivision);
                }
            }

            if let Some(show_upper) = search_info.show_upper_div {
                if show_upper {
                    query_builder =
                        query_builder.filter_courses_by(CourseLevelFilter::UpperDivision);
                }
            }

            if let Some(show_grad) = search_info.show_grad_div {
                if show_grad {
                    query_builder = query_builder.filter_courses_by(CourseLevelFilter::Graduate);
                }
            }

            // For times, we only permit adding it if both start and end times are specified.
            if let (Some(s_hr), Some(s_min)) = (
                search_info.start_hr.and_then(|h| u32::try_from(h).ok()),
                search_info.start_min.and_then(|m| u32::try_from(m).ok()),
            ) {
                query_builder = query_builder.set_start_time(s_hr, s_min);
            }

            if let (Some(e_hr), Some(e_min)) = (
                search_info.end_hr.and_then(|h| u32::try_from(h).ok()),
                search_info.end_min.and_then(|m| u32::try_from(m).ok()),
            ) {
                query_builder = query_builder.set_end_time(e_hr, e_min);
            }

            // And then, finally, apply the day of weeks
            if let Some(days) = search_info.days {
                for d in days {
                    query_builder = match d.as_str() {
                        "M" | "m" => query_builder.apply_day(DayOfWeek::Monday),
                        "Tu" | "tu" => query_builder.apply_day(DayOfWeek::Tuesday),
                        "W" | "we" => query_builder.apply_day(DayOfWeek::Wednesday),
                        "Th" | "th" => query_builder.apply_day(DayOfWeek::Thursday),
                        "F" | "f" => query_builder.apply_day(DayOfWeek::Friday),
                        "Sa" | "sa" => query_builder.apply_day(DayOfWeek::Saturday),
                        "Su" | "su" => query_builder.apply_day(DayOfWeek::Sunday),
                        _ => query_builder,
                    }
                }
            }

            let guard = term_info.general_wrapper.lock().await;
            process_return(
                guard
                    .search_courses(SearchType::Advanced(&query_builder))
                    .await,
            )
        },
        s,
    )
    .await
}
