-- This file should undo anything in `up.sql`
delete from users where username in ('admin', 'user');
