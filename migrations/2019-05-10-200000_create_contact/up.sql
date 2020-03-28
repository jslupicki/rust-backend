-- Your SQL goes here
CREATE TABLE contacts
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    employee_id   INTEGER                           NOT NULL,
    from_date     DATE                              NOT NULL,
    to_date       DATE                              NOT NULL,
    phone         TEXT                              NOT NULL,
    address       TEXT,
    search_string TEXT                              NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);
