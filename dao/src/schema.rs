table! {
    contacts (id) {
        id -> Integer,
        employee_id -> Integer,
        from_date -> Date,
        to_date -> Date,
        phone -> Text,
        address -> Nullable<Text>,
        search_string -> Text,
    }
}

table! {
    employees (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        search_string -> Text,
    }
}

table! {
    salaries (id) {
        id -> Integer,
        employee_id -> Integer,
        from_date -> Date,
        to_date -> Date,
        amount -> BigInt,
        search_string -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        is_admin -> Bool,
    }
}

joinable!(contacts -> employees (employee_id));
joinable!(salaries -> employees (employee_id));

allow_tables_to_appear_in_same_query!(contacts, employees, salaries, users,);
