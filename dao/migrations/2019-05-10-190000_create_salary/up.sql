-- Your SQL goes here
CREATE TABLE salaries
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    employee_id   INTEGER                           NOT NULL,
    from_date     DATE                              NOT NULL,
    to_date       DATE                              NOT NULL,
    amount        INTEGER                           NOT NULL,
    search_string TEXT                              NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

