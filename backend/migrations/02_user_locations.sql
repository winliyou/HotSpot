-- user_locations表 - 存储用户位置信息
-- 文件：02_user_locations.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS user_locations (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    location_name TEXT,
    geom GEOGRAPHY(POINT, 4326)
);

-- 添加触发器函数来自动更新地理点字段
CREATE OR REPLACE FUNCTION update_geom_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.geom = ST_SetSRID(ST_MakePoint(NEW.longitude, NEW.latitude), 4326)::geography;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_locations_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON user_locations
FOR EACH ROW EXECUTE FUNCTION update_geom_column();

-- 创建PostGIS空间索引
CREATE INDEX IF NOT EXISTS idx_user_locations_geom
ON user_locations USING gist (geom);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_user_locations_user_id ON user_locations(user_id);
CREATE INDEX IF NOT EXISTS idx_user_locations_updated_at ON user_locations(updated_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE user_locations ADD COLUMN id BIGSERIAL;
-- ALTER TABLE user_locations DROP CONSTRAINT user_locations_pkey CASCADE;
-- ALTER TABLE user_locations ADD PRIMARY KEY (id);