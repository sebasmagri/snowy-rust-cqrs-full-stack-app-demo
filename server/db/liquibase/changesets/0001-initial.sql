--liquibase formatted sql

--changeset snowy:1
--comment: initial schema for CQRS-ES
CREATE TABLE events
(
    aggregate_type text                         NOT NULL,
    aggregate_id   text                         NOT NULL,
    sequence       bigint CHECK (sequence >= 0) NOT NULL,
    event_type     text                         NOT NULL,
    event_version  text                         NOT NULL,
    payload        json                         NOT NULL,
    metadata       json                         NOT NULL,
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE team_query
(
    view_id text                                  NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

--rollback DROP TABLE events;
--rollback DROP TABLE account_query;