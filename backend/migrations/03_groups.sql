-- groups表 - 存储群组信息
-- 文件：03_groups.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS groups (
    id BIGSERIAL PRIMARY KEY,
    group_id UUID NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    owner_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_active_at TIMESTAMPTZ NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    location_name TEXT NOT NULL,
    password_hash TEXT,
    geom GEOGRAPHY(POINT, 4326)
);

-- 添加唯一约束
ALTER TABLE groups ADD CONSTRAINT groups_group_id_key UNIQUE (group_id);

-- 添加触发器函数来自动更新地理点字段
CREATE TRIGGER update_groups_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON groups
FOR EACH ROW EXECUTE FUNCTION update_geom_column();

-- 创建PostGIS空间索引
CREATE INDEX IF NOT EXISTS idx_groups_geom
ON groups USING gist (geom);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_groups_owner_id ON groups(owner_id);
CREATE INDEX IF NOT EXISTS idx_groups_last_active_at ON groups(last_active_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE groups ADD COLUMN id BIGSERIAL;
-- ALTER TABLE groups DROP CONSTRAINT groups_pkey CASCADE;
-- ALTER TABLE groups ADD PRIMARY KEY (id);
-- ALTER TABLE groups ADD CONSTRAINT groups_group_id_key UNIQUE (group_id);

-- 修复外键关系
-- ALTER TABLE group_members ADD CONSTRAINT group_members_group_id_fkey 
--     FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE CASCADE;
-- ALTER TABLE group_messages ADD CONSTRAINT group_messages_group_id_fkey 
--     FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE CASCADE; 