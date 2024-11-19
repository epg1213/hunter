CREATE TABLE parameter (
    id int NOT NULL AUTO_INCREMENT,
    page_id int,
    name VARCHAR(255),
    type VARCHAR(255),
    first_seen_value (255),
    PRIMARY KEY (id)
);
