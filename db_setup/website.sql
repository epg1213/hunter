CREATE TABLE website (
    id int NOT NULL AUTO_INCREMENT,
    project_id int,
    name VARCHAR(255),
    added_date DATETIME,
    PRIMARY KEY (id)
);
