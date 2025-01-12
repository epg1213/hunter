DROP TABLE IF EXISTS hunting.page

CREATE TABLE page (
    id int NOT NULL AUTO_INCREMENT,
    website_id int,
    name VARCHAR(255),
    max_content_length int,
    redirects VARCHAR(255),
    PRIMARY KEY (id)
);
