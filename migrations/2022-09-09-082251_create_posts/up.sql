CREATE TYPE "PostStatus" as ENUM (
    'draft',
    'published',
    'deleted'
);

CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  status "PostStatus" NOT NULL DEFAULT 'draft',
  col1 INT,
  col2 INT
)
