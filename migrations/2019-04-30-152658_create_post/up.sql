-- Your SQL goes here
CREATE TABLE posts (
  id INT UNSIGNED AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  body VARCHAR(255) NOT NULL DEFAULT '',
  published BOOL NOT NULL DEFAULT 0,
  PRIMARY KEY (id)
)
