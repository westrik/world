ALTER TABLE jobs
    ADD COLUMN user_id INT,
    ADD FOREIGN KEY (user_id) REFERENCES users (id);
