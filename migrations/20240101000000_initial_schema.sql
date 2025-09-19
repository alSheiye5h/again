-- Create a custom type for user roles in communities, if it doesn't exist.
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'member_role') THEN
        CREATE TYPE member_role AS ENUM ('member', 'staff', 'admin');
    END IF;
END$$;

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100),
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    profil_pic VARCHAR(255),
    bio TEXT,
    deleted BOOLEAN DEFAULT FALSE
);

-- Create low_discussion table for all discussion threads
CREATE TABLE IF NOT EXISTS low_discussion (
    id SERIAL PRIMARY KEY
);

-- Create post table
CREATE TABLE IF NOT EXISTS post (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    discussion_id INT REFERENCES low_discussion(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create post_interaction table
CREATE TABLE IF NOT EXISTS post_interaction (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id INT NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    interaction_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, post_id, interaction_type)
);

-- Create other tables like club, community, etc. here if they don't have migrations yet.
-- For now, this covers the core post functionality.
