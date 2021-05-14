-- Your SQL goes here

CREATE TABLE IF NOT EXISTS user_profiles (
                                        id UUID NOT NULL DEFAULT uuid_generate_v4(),

                                        created_by UUID NULL,
                                        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                        updated_by UUID NULL,
                                        updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                        deleted_at TIMESTAMP NULL,
                                        delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                                        PRIMARY KEY (id)
);

-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- ALTER TABLE accounts
--   ALTER COLUMN last_login_ip DROP NOT NULL,
--   ALTER COLUMN current_login_ip DROP NOT NULL;