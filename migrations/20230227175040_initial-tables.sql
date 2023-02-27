CREATE TABLE IF NOT EXISTS lottery.lotteries (
    id SERIAL PRIMARY KEY,
    owner_id VARCHAR ( 40 ) NOT NULL,
    guild_id VARCHAR ( 40 ) NOT NULL,
    start_timestamp TIMESTAMP WITH TIME ZONE NOT NULL ,
    duration_days SMALLINT NOT NULL DEFAULT 7,
    ended BOOLEAN NOT NULL DEFAULT false,
    pot BIGINT NOT NULL DEFAULT 0,
    price INT NOT NULL
);

CREATE TABLE IF NOT EXISTS lottery.players (
    id SERIAL PRIMARY KEY,
    lottery_id INTEGER NOT NULL,
    member_id VARCHAR ( 40 ) NOT NULL,
    FOREIGN KEY ( lottery_id ) REFERENCES lottery.lotteries ( id )
);