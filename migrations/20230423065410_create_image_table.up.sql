-- Add up migration script here
CREATE TABLE IF NOT EXISTS image (
    id              INTEGER PRIMARY KEY autoincrement,
    prompt          TEXT    NOT NULL,
    negative_prompt TEXT    NOT NULL,
    steps           INTEGER NOT NULL,
    sampler         TEXT    NOT NULL,
    cfg_scale       REAL    NOT NULL,
    seed            INTEGER NOT NULL,
    width           INTEGER NOT NULL,
    height          INTEGER NOT NULL,
    model_hash      TEXT    NOT NULL,
    model           TEXT    NOT NULL,
    clip_skip       INTEGER NOT NULL,
    file_path       TEXT    NULL
);
