table! {
    party (id) {
        id -> Binary,
        party_subtype -> Enum,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        body -> Varchar,
        published -> Bool,
    }
}

table! {
    work_effort (id) {
        id -> Binary,
        parent_work_effort_id -> Binary,
        prerequisite_work_effort_id -> Binary,
        work_effort_type_id -> Binary,
        name -> Varchar,
        scheduled_start_date -> Date,
        scheduled_end_date -> Date,
    }
}

table! {
    work_effort_type (id) {
        id -> Binary,
        parent_work_effort_type_id -> Binary,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    party,
    posts,
    work_effort,
    work_effort_type,
);
