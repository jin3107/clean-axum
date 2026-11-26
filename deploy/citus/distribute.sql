-- Run once on the coordinator, after the app has applied its migrations.
BEGIN;

-- small lookup table, copied to every node
SELECT create_reference_table('tenant');

-- Citus rejects a foreign key from a local table to a distributed one, so the
-- blog -> user key is dropped while the two tables are distributed one by one.
ALTER TABLE blog DROP CONSTRAINT fk_blog_author;

-- one tenant's rows share a shard, so joins and foreign keys stay on one node
SELECT create_distributed_table('"user"', 'tenant_id');
SELECT create_distributed_table('blog', 'tenant_id', colocate_with => '"user"');

-- same definition as the migration
ALTER TABLE blog
    ADD CONSTRAINT fk_blog_author FOREIGN KEY (tenant_id, author_id)
    REFERENCES "user" (tenant_id, id) ON DELETE CASCADE;

COMMIT;

SELECT table_name, citus_table_type, distribution_column, colocation_id
FROM citus_tables
ORDER BY table_name::text;
