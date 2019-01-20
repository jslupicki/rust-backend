-- Default users user/user and admin/admin - password as SHA3 256
insert into main.users(username, password, is_admin) values ('user', '8ac76453d769d4fd14b3f41ad4933f9bd64321972cd002de9b847e117435b08b', 'f');
insert into main.users(username, password, is_admin) values ('admin', 'fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b', 't');
