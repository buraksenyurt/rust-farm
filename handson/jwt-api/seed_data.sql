CREATE TABLE IF NOT EXISTS users
(
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR(25),
    password VARCHAR(250),
    role VARCHAR(15),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);

CREATE TABLE IF NOT EXISTS categories
(
    id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR(50),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);

INSERT INTO categories (title)
SELECT 'Books'
WHERE NOT EXISTS (
    SELECT NULL FROM categories
    WHERE (title) = ('Books')
);

INSERT INTO categories (title)
SELECT 'Longplays'
WHERE NOT EXISTS (
    SELECT NULL FROM categories
    WHERE (title) = ('Longplays')
);