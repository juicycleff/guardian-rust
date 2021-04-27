-- Your SQL goes here

CREATE TABLE IF NOT EXISTS refresh_tokens (
                                id UUID NOT NULL DEFAULT uuid_generate_v4(),
                                account_id UUID NOT NULL,
                                token VARCHAR(122) NOT NULL,

                                created_by UUID NOT NULL,
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_by UUID NOT NULL,
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                deleted_at TIMESTAMP NULL,
                                delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                                CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id),

                                PRIMARY KEY (id),
                                UNIQUE(token)
);