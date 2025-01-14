use hunting;

DROP TABLE IF EXISTS hunting.project;
CREATE TABLE project (
    id int NOT NULL AUTO_INCREMENT,
    name VARCHAR(255),
    creation_date DATETIME,
    PRIMARY KEY (id)
);
