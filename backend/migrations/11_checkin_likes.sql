-- checkin_likes表 - 存储签到点赞记录
-- 文件：11_checkin_likes.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS checkin_likes (
    id BIGSERIAL PRIMARY KEY,
    checkin_id UUID NOT NULL REFERENCES checkins(checkin_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL
);

-- 添加唯一约束
ALTER TABLE checkin_likes ADD CONSTRAINT checkin_likes_checkin_user_unique UNIQUE (checkin_id, user_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_checkin_likes_checkin_id ON checkin_likes(checkin_id);
CREATE INDEX IF NOT EXISTS idx_checkin_likes_user_id ON checkin_likes(user_id);
CREATE INDEX IF NOT EXISTS idx_checkin_likes_created_at ON checkin_likes(created_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE checkin_likes ADD COLUMN id BIGSERIAL;
-- ALTER TABLE checkin_likes DROP CONSTRAINT checkin_likes_pkey;
-- ALTER TABLE checkin_likes ADD PRIMARY KEY (id);
-- ALTER TABLE checkin_likes ADD CONSTRAINT checkin_likes_checkin_user_unique UNIQUE (checkin_id, user_id); 