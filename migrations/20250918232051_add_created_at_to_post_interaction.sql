-- Add migration script here
-- Add created_at column to post_interaction table
ALTER TABLE post_interaction
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

