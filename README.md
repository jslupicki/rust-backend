### This is my toy project to learn Rust.

Some time ago I've participated in recruit process in Solteq. I've ended up in different company but
I'm considering this as one of the best recruitment experience in which I've participated. 
Instead interviews, whiteboard test or so, they give me specification of simple demo application to manage
employees and few days to make it. It take maybe 8 hours to finish it, prepare docker image and publish it on AWS. 
I've use Java, Spring Boot and H2 database plus Vaadin to create rich web application and backend.

To learn Rust I've decided using specification of this demo application for Solteq as it was simple
but not trivial and give me opportunity to learn. 
Here is REST backend for application and for the future there is plan to create also
front end in Rust (web assembly target + Yew or similar framework).

Used tools/frameworks:
* **actix-web** to expose REST interface
* **diesel** as ORM to manage DB operations
* **SQLite** as database

What is done:
* access control through actix-web middleware and cookie based sessions.
* user management (CRUD operation on users).
* DAO backend for users, employees, salaries and contacts.
* quite nice integration tests set up.
 
What is not yet finished:
* employee management. Rest endpoint are just mocked and responds with "NOT YET IMPLEMENTED".
DAO should be rewrite to handle employee, salaries and contacts as one object (EmployeeDTO) 
in one transaction.
  

