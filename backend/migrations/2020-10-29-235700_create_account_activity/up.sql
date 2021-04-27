-- Your SQL goes here

CREATE TABLE IF NOT EXISTS account_activities (
                                id UUID NOT NULL DEFAULT uuid_generate_v4(),
                                account_id UUID NOT NULL,
                                log_name VARCHAR NULL,
                                description VARCHAR NOT NULL,
                                subject_id UUID NULL,
                                subject_type VARCHAR NULL,
                                causer_id UUID NULL,
                                causer_type VARCHAR NULL,
                                properties JSON NULL,

                                created_by UUID NOT NULL,
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_by UUID NOT NULL,
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                deleted_at TIMESTAMP NULL,
                                delete_flag BOOLEAN NOT NULL DEFAULT FALSE,

                                PRIMARY KEY (id),
                                CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id),
                                UNIQUE(log_name),
                                UNIQUE(subject_id, subject_type),
                                UNIQUE(causer_id, causer_type)
);