-- checkins表 - 存储用户签到信息
-- 文件：09_checkins.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS checkins (
    id BIGSERIAL PRIMARY KEY,
    checkin_id UUID NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    description TEXT,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    location_name TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    geom GEOGRAPHY(POINT, 4326)
);

-- 添加唯一约束
ALTER TABLE checkins ADD CONSTRAINT checkins_checkin_id_key UNIQUE (checkin_id);

-- 添加触发器函数来自动更新地理点字段
CREATE TRIGGER update_checkins_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON checkins
FOR EACH ROW EXECUTE FUNCTION update_geom_column();

-- 创建PostGIS空间索引
CREATE INDEX IF NOT EXISTS idx_checkins_geom
ON checkins USING gist (geom);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_checkins_user_id ON checkins(user_id);
CREATE INDEX IF NOT EXISTS idx_checkins_created_at ON checkins(created_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE checkins ADD COLUMN id BIGSERIAL;
-- ALTER TABLE checkins DROP CONSTRAINT checkins_pkey CASCADE;
-- ALTER TABLE checkins ADD PRIMARY KEY (id);
-- ALTER TABLE checkins ADD CONSTRAINT checkins_checkin_id_key UNIQUE (checkin_id);

-- 修复外键关系
-- ALTER TABLE checkin_tags ADD CONSTRAINT checkin_tags_checkin_id_fkey 
--     FOREIGN KEY (checkin_id) REFERENCES checkins(checkin_id) ON DELETE CASCADE;
-- ALTER TABLE checkin_likes ADD CONSTRAINT checkin_likes_checkin_id_fkey 
--     FOREIGN KEY (checkin_id) REFERENCES checkins(checkin_id) ON DELETE CASCADE; 