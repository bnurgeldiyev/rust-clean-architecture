DROP TABLE IF EXISTS tbl_user;

CREATE TABLE tbl_user (
    id SERIAL  PRIMARY KEY,
    username  varchar(100) NOT NULL,
    password  varchar(150) NOT NULL,
    firstname varchar(50)  NOT NULL,
    lastname  varchar(50)  NOT NULL,
    create_ts bigint NOT NULL DEFAULT trunc(extract(epoch from now() at time zone 'UTC-5')),
    update_ts bigint NOT NULL DEFAULT trunc(extract(epoch from now() at time zone 'UTC-5'))
);

INSERT INTO tbl_user(username, password, firstname, lastname) VALUES('bnurgeldiyev', '$2b$12$BKtf7ryLETJwczyXEm.t0uALGTt1i5xXB8Gzxn7Nkrs4FGXqn/WBm', 'Batyr', 'Nurgeldiyev');
