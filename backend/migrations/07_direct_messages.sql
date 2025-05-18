-- direct_messages表 - 存储私聊消息
-- 文件：07_direct_messages.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS direct_messages (
    id BIGSERIAL PRIMARY KEY,
    message_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(conversation_id) ON DELETE CASCADE,
    sender_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    recipient_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text',
    created_at TIMESTAMPTZ NOT NULL,
    read_at TIMESTAMPTZ,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    geom GEOGRAPHY(POINT, 4326)
);

-- 添加唯一约束
ALTER TABLE direct_messages ADD CONSTRAINT direct_messages_message_id_key UNIQUE (message_id);

-- 添加触发器函数来自动更新地理点字段
CREATE TRIGGER update_direct_messages_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON direct_messages
FOR EACH ROW EXECUTE FUNCTION update_geom_column();

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_direct_messages_conversation_id ON direct_messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_direct_messages_created_at ON direct_messages(created_at);
CREATE INDEX IF NOT EXISTS idx_direct_messages_sender_id ON direct_messages(sender_id);
CREATE INDEX IF NOT EXISTS idx_direct_messages_recipient_id ON direct_messages(recipient_id);
CREATE INDEX IF NOT EXISTS idx_direct_messages_read_at ON direct_messages(read_at);

-- 创建地理位置索引
CREATE INDEX IF NOT EXISTS idx_direct_messages_geom
ON direct_messages USING gist (geom);

-- 修改 direct_messages 表的外键
-- ALTER TABLE direct_messages DROP CONSTRAINT IF EXISTS direct_messages_sender_id_fkey;
-- ALTER TABLE direct_messages DROP CONSTRAINT IF EXISTS direct_messages_recipient_id_fkey;
-- ALTER TABLE direct_messages ADD COLUMN sender_uuid UUID;
-- ALTER TABLE direct_messages ADD COLUMN recipient_uuid UUID;
-- UPDATE direct_messages dm SET sender_uuid = s.uuid, recipient_uuid = r.uuid FROM users s, users r WHERE dm.sender_id = s.user_id AND dm.recipient_id = r.user_id;
-- ALTER TABLE direct_messages ALTER COLUMN sender_uuid SET NOT NULL;
-- ALTER TABLE direct_messages ALTER COLUMN recipient_uuid SET NOT NULL;
-- ALTER TABLE direct_messages DROP COLUMN sender_id;
-- ALTER TABLE direct_messages DROP COLUMN recipient_id;
-- ALTER TABLE direct_messages RENAME COLUMN sender_uuid TO sender_id;
-- ALTER TABLE direct_messages RENAME COLUMN recipient_uuid TO recipient_id;
-- ALTER TABLE direct_messages ADD CONSTRAINT direct_messages_sender_id_fkey FOREIGN KEY (sender_id) REFERENCES users(uuid) ON DELETE CASCADE;
-- ALTER TABLE direct_messages ADD CONSTRAINT direct_messages_recipient_id_fkey FOREIGN KEY (recipient_id) REFERENCES users(uuid) ON DELETE CASCADE; 