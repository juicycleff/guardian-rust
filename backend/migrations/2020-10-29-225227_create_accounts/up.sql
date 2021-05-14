-- Your SQL goes here

CREATE TABLE IF NOT EXISTS accounts (
                                        id UUID NOT NULL DEFAULT uuid_generate_v4(),
                                        username VARCHAR(100) NULL,
                                        email VARCHAR(100) NOT NULL,
                                        password VARCHAR(122) NOT NULL,

                                        last_login_at TIMESTAMP NULL,
                                        current_login_at TIMESTAMP NULL,
                                        confirmed_at TIMESTAMP NULL,
                                        locked_at TIMESTAMP NULL,
                                        confirmation_sent_at TIMESTAMP NULL,
                                        password_changed_at TIMESTAMP NULL,
                                        remember_created_at TIMESTAMP NULL,
                                        reset_password_created_at TIMESTAMP NULL,

                                        login_count_at INT NULL,
                                        failed_attempts INT NULL,

                                        reset_password_token VARCHAR NULL,
                                        confirmation_token VARCHAR NULL,
                                        unlock_token VARCHAR NULL,

                                        last_login_ip VARCHAR NULL,
                                        current_login_ip VARCHAR NULL,
                                        unconfirmed_email VARCHAR NULL,

                                        locked BOOLEAN NOT NULL DEFAULT FALSE,
                                        require_new_password BOOLEAN NOT NULL DEFAULT FALSE,

                                        created_by UUID NULL,
                                        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                        updated_by UUID NULL,
                                        updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                        deleted_at TIMESTAMP NULL,
                                        delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                                        PRIMARY KEY (id),
                                        UNIQUE(email),
                                        UNIQUE(username),
                                        UNIQUE(confirmation_token),
                                        UNIQUE(unlock_token),
                                        UNIQUE(reset_password_token)
);

-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- ALTER TABLE accounts
--   ALTER COLUMN last_login_ip DROP NOT NULL,
--   ALTER COLUMN current_login_ip DROP NOT NULL;