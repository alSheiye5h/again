-- Add migration script here
ALTER TABLE team_members ADD CONSTRAINT team_members_team_id_user_id_key UNIQUE (team_id, user_id);
