CREATE TABLE IF NOT EXISTS balance_tips
(
    key bytea PRIMARY KEY,
    value bytea NOT NULL,
    last_changed_notebook integer NOT NULL
);


CREATE TABLE IF NOT EXISTS block_sync_lock (
    key integer PRIMARY KEY
);

INSERT INTO block_sync_lock (key) VALUES (1);

CREATE TABLE  IF NOT EXISTS blocks (
    block_hash bytea NOT NULL PRIMARY KEY,
    parent_hash bytea NOT NULL,
    block_number integer NOT NULL,
    block_vote_minimum varchar NOT NULL,
    this_notary_notebook_number integer,
    parent_voting_key bytea NULL,
    is_finalized boolean NOT NULL,
    finalized_time timestamptz,
    received_time timestamptz NOT NULL
);

CREATE TABLE IF NOT EXISTS chain_transfers
(
    to_localchain boolean NOT NULL,
    account_id bytea NOT NULL,
    account_nonce integer NULL,
    amount varchar NOT NULL,
    finalized_block_number integer NULL,
    included_in_notebook_number integer NULL
);

CREATE INDEX IF NOT EXISTS chain_transfers_included_in_notebook_number ON chain_transfers (included_in_notebook_number);


CREATE TABLE  IF NOT EXISTS registered_keys (
    public bytea PRIMARY KEY,
    finalized_block_number integer NOT NULL
);
CREATE TABLE IF NOT EXISTS notarizations (
     notebook_number integer NOT NULL,
     balance_changes jsonb NOT NULL,
     block_votes jsonb NOT NULL
);

CREATE INDEX IF NOT EXISTS notarizations_notebook_number ON notarizations (notebook_number);

CREATE TABLE IF NOT EXISTS notebook_origins (
    account_id bytea NOT NULL,
    account_type INTEGER NOT NULL,
    uid INTEGER NOT NULL,
    notebook_number INTEGER NOT NULL,
    PRIMARY KEY (account_id, account_type)
);

CREATE UNIQUE INDEX IF NOT EXISTS notebook_origins_uid_notebook_number ON notebook_origins (uid, notebook_number);


CREATE TABLE IF NOT EXISTS notebooks (
    notebook_number INTEGER PRIMARY KEY NOT NULL,
    new_account_origins jsonb NOT NULL,
    change_merkle_leafs BYTEA[] NOT NULL,
    block_votes jsonb NOT NULL,
    hash BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    last_updated timestamptz NOT NULL default now()
);

CREATE TABLE IF NOT EXISTS notebooks_raw (
     notebook_number INTEGER PRIMARY KEY NOT NULL,
     encoded BYTEA NOT NULL
);

CREATE TABLE IF NOT EXISTS notebook_secrets (
    notebook_number integer NOT NULL PRIMARY KEY,
    secret bytea NOT NULL
);

CREATE TABLE IF NOT EXISTS notebook_headers (
    notebook_number INTEGER PRIMARY KEY NOT NULL,
    version INTEGER NOT NULL,
    hash BYTEA,
    block_number INTEGER NOT NULL,
    finalized_block_number INTEGER,
    start_time timestamptz NOT NULL,
    end_time timestamptz NULL,
    notary_id INTEGER NOT NULL,
    tax varchar,
    chain_transfers jsonb NOT NULL,
    changed_accounts_root BYTEA NOT NULL,
    changed_account_origins jsonb NOT NULL,
    block_votes_root BYTEA NOT NULL,
    block_votes_count integer NOT NULL,
    block_voting_power varchar NOT NULL,
    blocks_with_votes bytea[] NOT NULL,
    secret_hash BYTEA NOT NULL,
    parent_secret BYTEA NULL,
    best_nonces jsonb NOT NULL,
    last_updated timestamptz NOT NULL default now()
);

CREATE TABLE IF NOT EXISTS notebook_status (
    notebook_number INTEGER NOT NULL,
    chain_transfers INTEGER NOT NULL default 0,
    block_votes INTEGER NOT NULL default 0,
    balance_changes INTEGER NOT NULL default 0,
    notarizations INTEGER NOT NULL default 0,
    step INTEGER NOT NULL,
    open_time timestamptz NOT NULL,
    ready_for_close_time timestamptz NULL,
    closed_time timestamptz NULL,
    submitted_time timestamptz NULL,
    finalized_time timestamptz NULL
);

CREATE UNIQUE INDEX idx_one_open_notebook
    ON notebook_status (step)
    WHERE step = 1;

-- Do not allow a notebook to be modified once it has been finalized
CREATE OR REPLACE FUNCTION check_notebook_finalized()
    RETURNS TRIGGER AS $$
BEGIN
    IF OLD.end_time IS NOT NULL THEN
        RAISE EXCEPTION 'This notebook header is finalized and immutable';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE OR REPLACE FUNCTION update_last_modified()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER immutable_finalized_notebook
    BEFORE UPDATE ON notebook_headers
    FOR EACH ROW
EXECUTE PROCEDURE check_notebook_finalized();

CREATE TRIGGER update_header_last_modified
    BEFORE UPDATE ON notebook_headers
    FOR EACH ROW
EXECUTE PROCEDURE update_last_modified();

CREATE TRIGGER update_notebook_last_modified
    BEFORE UPDATE ON notebooks
    FOR EACH ROW
EXECUTE PROCEDURE update_last_modified();


-- Create 5 sequences so that we can safely close a notebook without any overlap
-- The sequence in use at any given moment is notebook_number % 5

CREATE SEQUENCE IF NOT EXISTS uid_seq_0;
CREATE SEQUENCE IF NOT EXISTS uid_seq_1;
CREATE SEQUENCE IF NOT EXISTS uid_seq_2;
CREATE SEQUENCE IF NOT EXISTS uid_seq_3;
CREATE SEQUENCE IF NOT EXISTS uid_seq_4;

-- TODO: need to know the roles
-- GRANT USAGE ON SEQUENCE uid_seq_0 TO ulx;
-- GRANT USAGE ON SEQUENCE uid_seq_1 TO ulx;
-- GRANT USAGE ON SEQUENCE uid_seq_2 TO ulx;
-- GRANT USAGE ON SEQUENCE uid_seq_3 TO ulx;
-- GRANT USAGE ON SEQUENCE uid_seq_4 TO ulx;
