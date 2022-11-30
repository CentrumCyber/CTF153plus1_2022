CREATE TABLE app_user(
	id INTEGER PRIMARY KEY AUTO_INCREMENT,
	username CHAR(20) NOT NULL,
	password CHAR(64) NOT NULL,
	note CHAR(255) NOT NULL DEFAULT ''
);

INSERT INTO app_user(
	username,
	password,
	note
) VALUES(
	'admin',
	'________________________________________________________________',
	'flag{var_1s_go0d_1n_sql_unl1ke_1n_js}'
);