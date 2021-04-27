-- Your SQL goes here

CREATE TABLE IF NOT EXISTS oauth_accounts (
                          id UUID NOT NULL DEFAULT uuid_generate_v4(),
                          account_id UUID NOT NULL,
                          provider VARCHAR(100) NOT NULL,
                          provider_id VARCHAR(122) NOT NULL,
                          access_token VARCHAR(122) NOT NULL,

                          created_by VARCHAR(36) NOT NULL,
                          created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          updated_by VARCHAR(36) NOT NULL,
                          updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          deleted_at TIMESTAMP NULL,
                          delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                          PRIMARY KEY (id),
                          CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id)
);