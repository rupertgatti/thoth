table! {
    use diesel::sql_types::*;
    use crate::models::*;

    contribution (work_id, contributor_id, contribution_type) {
        work_id -> Uuid,
        contributor_id -> Uuid,
        contribution_type -> Contribution_type,
        main_contribution -> Bool,
        biography -> Nullable<Text>,
        institution -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    contributor (contributor_id) {
        contributor_id -> Uuid,
        first_name -> Nullable<Text>,
        last_name -> Text,
        full_name -> Text,
        orcid -> Nullable<Text>,
        website -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    issue (series_id, work_id) {
        series_id -> Uuid,
        work_id -> Uuid,
        issue_ordinal -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    publication (publication_id) {
        publication_id -> Uuid,
        publication_type -> Publication_type,
        work_id -> Uuid,
        isbn -> Nullable<Text>,
        publication_url -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    publisher (publisher_id) {
        publisher_id -> Uuid,
        publisher_name -> Text,
        publisher_shortname -> Nullable<Text>,
        publisher_url -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    series (series_id) {
        series_id -> Uuid,
        series_type -> Series_type,
        series_name -> Text,
        issn_print -> Text,
        issn_digital -> Text,
        series_url -> Nullable<Text>,
        publisher_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    work (work_id) {
        work_id -> Uuid,
        work_type -> Work_type,
        full_title -> Text,
        title -> Text,
        subtitle -> Nullable<Text>,
        publisher_id -> Uuid,
        doi -> Nullable<Text>,
        publication_date -> Nullable<Date>,
    }
}

joinable!(contribution -> contributor (contributor_id));
joinable!(contribution -> work (work_id));
joinable!(issue -> series (series_id));
joinable!(issue -> work (work_id));
joinable!(publication -> work (work_id));
joinable!(series -> publisher (publisher_id));
joinable!(work -> publisher (publisher_id));

allow_tables_to_appear_in_same_query!(
    contribution,
    contributor,
    issue,
    publication,
    publisher,
    series,
    work,
);
