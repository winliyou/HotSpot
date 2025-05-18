-- checkin_tags表 - 存储签到与标签的关联
-- 文件：10_checkin_tags.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS checkin_tags (
    id BIGSERIAL PRIMARY KEY,
    checkin_id UUID NOT NULL REFERENCES checkins(checkin_id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(tag_id) ON DELETE CASCADE
);

-- 添加唯一约束
ALTER TABLE checkin_tags ADD CONSTRAINT checkin_tags_checkin_tag_unique UNIQUE (checkin_id, tag_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_checkin_tags_checkin_id ON checkin_tags(checkin_id);
CREATE INDEX IF NOT EXISTS idx_checkin_tags_tag_id ON checkin_tags(tag_id);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE checkin_tags ADD COLUMN id BIGSERIAL;
-- ALTER TABLE checkin_tags DROP CONSTRAINT checkin_tags_pkey;
-- ALTER TABLE checkin_tags ADD PRIMARY KEY (id);
-- ALTER TABLE checkin_tags ADD CONSTRAINT checkin_tags_checkin_tag_unique UNIQUE (checkin_id, tag_id); 