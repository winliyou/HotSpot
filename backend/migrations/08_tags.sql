-- tags表 - 存储标签信息
-- 文件：08_tags.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS tags (
    id BIGSERIAL PRIMARY KEY,
    tag_id UUID NOT NULL,
    name VARCHAR(50) NOT NULL UNIQUE,
    counter INTEGER NOT NULL DEFAULT 0
);

-- 添加唯一约束
ALTER TABLE tags ADD CONSTRAINT tags_tag_id_key UNIQUE (tag_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_counter ON tags(counter DESC);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE tags ADD COLUMN id BIGSERIAL;
-- ALTER TABLE tags DROP CONSTRAINT tags_pkey;
-- ALTER TABLE tags ADD PRIMARY KEY (id);
-- ALTER TABLE tags ADD CONSTRAINT tags_tag_id_key UNIQUE (tag_id); 