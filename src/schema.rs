// @generated automatically by Diesel CLI.

diesel::table! {
    gusts (key) {
        #[max_length = 200]
        key -> Varchar,
        content -> Bytea,
        expires -> Nullable<Timestamp>,
        anonymous -> Nullable<Bool>,
        is_volatile -> Nullable<Bool>,
        reads_remaining -> Nullable<Int4>,
        #[max_length = 8]
        visibility -> Varchar,
        created_at -> Timestamp,
        accessed -> Nullable<Int4>,
        starred -> Nullable<Int4>,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 200]
        language -> Varchar,
    }
}
