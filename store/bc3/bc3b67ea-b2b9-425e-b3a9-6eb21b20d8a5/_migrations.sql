ATTACH TABLE _ UUID 'f8b13d2b-bb9f-4df3-acf6-830c6a453c8a'
(
    `uid` UUID DEFAULT generateUUIDv4(),
    `version` UInt32,
    `checksum` String,
    `migration_name` String,
    `applied_at` DateTime DEFAULT now()
)
ENGINE = MergeTree
ORDER BY tuple(applied_at)
SETTINGS index_granularity = 8192
