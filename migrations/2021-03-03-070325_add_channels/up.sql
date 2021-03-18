-- Your SQL goes here
CREATE TABLE channels (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL ,
    updated_at TIMESTAMP NOT NULL--,
    --CONSTRAINT "fk_channel_message" FOREIGN KEY (messages, id) REFERENCES messages (id, channel) DEFERRABLE
);

--ALTER TABLE messages ADD FOREIGN KEY (id) REFERENCES channels (messages)

CREATE TABLE channel_messages (
    channel_id uuid REFERENCES channels (id),
    message_id uuid REFERENCES messages (id),
    iter BIGSERIAL NOT NULL
);
