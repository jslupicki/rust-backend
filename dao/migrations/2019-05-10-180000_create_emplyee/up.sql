-- Your SQL goes here
CREATE TABLE employees
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    first_name    TEXT                              NOT NULL,
    last_name     TEXT                              NOT NULL,
    search_string TEXT                              NOT NULL
)