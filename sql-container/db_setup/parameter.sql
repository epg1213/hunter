DROP TABLE IF EXISTS hunting.parameter

CREATE TABLE parameter (
    id int NOT NULL AUTO_INCREMENT,
    page_id int,
    name VARCHAR(255),
    type VARCHAR(255),
    first_seen_value VARCHAR(255),
    PRIMARY KEY (id)
);
