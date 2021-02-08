table! {
    competencies (id) {
        id -> Int4,
        component_id -> Int4,
        rank -> Int4,
        title -> Text,
        c1 -> Nullable<Text>,
        c2 -> Nullable<Text>,
        c3 -> Nullable<Text>,
        c4 -> Nullable<Text>,
    }
}

table! {
    components (id) {
        id -> Int4,
        domain_id -> Int4,
        rank -> Int4,
        title -> Text,
    }
}

table! {
    domains (id) {
        id -> Int4,
        rank -> Int4,
        title -> Text,
    }
}

table! {
    eleves (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        birthdate -> Date,
        school_entry -> Date,
        cycle -> Text,
        active -> Bool,
    }
}

table! {
    evaluations (id) {
        id -> Int4,
        eleve_id -> Int4,
        competency_id -> Int4,
        status -> Text,
        comment -> Nullable<Text>,
    }
}

table! {
    general_comments (id) {
        id -> Int4,
        eleve_id -> Int4,
        comment -> Nullable<Text>,
    }
}

table! {
    key_store (name) {
        name -> Text,
        value -> Nullable<Text>,
    }
}

joinable!(competencies -> components (component_id));
joinable!(components -> domains (domain_id));
joinable!(evaluations -> competencies (competency_id));
joinable!(evaluations -> eleves (eleve_id));
joinable!(general_comments -> eleves (eleve_id));

allow_tables_to_appear_in_same_query!(
    competencies,
    components,
    domains,
    eleves,
    evaluations,
    general_comments,
    key_store,
);
