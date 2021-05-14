-- Your SQL goes here

CREATE TABLE IF NOT EXISTS two_factor_phone_device (
                          id UUID NOT NULL DEFAULT uuid_generate_v4(),
                          name VARCHAR(100) NOT NULL,
                          number VARCHAR(100) NOT NULL,
                          key VARCHAR(122) NOT NULL,
                          method VARCHAR(122) NOT NULL,
                          account_id UUID NOT NULL,

                          confirmed BOOLEAN NOT NULL DEFAULT FALSE,

                          created_by UUID NOT NULL,
                          created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          updated_by UUID NOT NULL,
                          updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          deleted_at TIMESTAMP NULL,
                          delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                          PRIMARY KEY (id),
                          CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id)
);