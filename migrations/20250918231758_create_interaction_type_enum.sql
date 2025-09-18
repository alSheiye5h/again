-- Add migration script here
-- Create the custom ENUM type for post interactions.
CREATE TYPE interaction_type AS ENUM ('like', 'upvote', 'downvote', 'repost', 'share');

-- Alter the post_interaction table to use the new ENUM type.
-- This command temporarily changes the column type to text to drop the old constraint,
-- then converts it to the new ENUM type.
ALTER TABLE post_interaction
DROP CONSTRAINT IF EXISTS post_interaction_interaction_type_check,
ALTER COLUMN interaction_type TYPE interaction_type
USING interaction_type::interaction_type;