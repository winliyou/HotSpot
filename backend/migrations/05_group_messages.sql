-- group_messages表 - 存储群组消息
-- 文件：05_group_messages.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS group_messages (
    id BIGSERIAL PRIMARY KEY,
    message_id UUID NOT NULL,
    group_id UUID NOT NULL REFERENCES groups(group_id) ON DELETE CASCADE,
    sender_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text',
    created_at TIMESTAMPTZ NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    geom GEOGRAPHY(POINT, 4326)
);

-- 添加唯一约束
ALTER TABLE group_messages ADD CONSTRAINT group_messages_message_id_key UNIQUE (message_id);

-- 添加触发器函数来自动更新地理点字段
CREATE TRIGGER update_group_messages_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON group_messages
FOR EACH ROW EXECUTE FUNCTION update_geom_column();

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_group_messages_group_id ON group_messages(group_id);
CREATE INDEX IF NOT EXISTS idx_group_messages_created_at ON group_messages(created_at);
CREATE INDEX IF NOT EXISTS idx_group_messages_sender_id ON group_messages(sender_id);

-- 创建地理位置索引
CREATE INDEX IF NOT EXISTS idx_group_messages_geom
ON group_messages USING gist (geom);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE group_messages ADD COLUMN id BIGSERIAL;
-- ALTER TABLE group_messages DROP CONSTRAINT group_messages_pkey CASCADE;
-- ALTER TABLE group_messages ADD PRIMARY KEY (id);
-- ALTER TABLE group_messages ADD CONSTRAINT group_messages_message_id_key UNIQUE (message_id);

-- 修改发送者ID字段类型的历史操作
-- ALTER TABLE group_messages DROP CONSTRAINT IF EXISTS group_messages_sender_id_fkey;
-- ALTER TABLE group_messages ADD COLUMN sender_uuid UUID;
-- UPDATE group_messages gm SET sender_uuid = u.uuid FROM users u WHERE gm.sender_id = u.user_id;
-- ALTER TABLE group_messages ALTER COLUMN sender_uuid SET NOT NULL;
-- ALTER TABLE group_messages DROP COLUMN sender_id;
-- ALTER TABLE group_messages RENAME COLUMN sender_uuid TO sender_id;
-- ALTER TABLE group_messages ADD CONSTRAINT group_messages_sender_id_fkey FOREIGN KEY (sender_id) REFERENCES users(uuid) ON DELETE CASCADE; 