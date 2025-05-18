-- conversations表 - 存储会话信息（用于私聊）
-- 文件：06_conversations.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS conversations (
    id BIGSERIAL PRIMARY KEY,
    conversation_id UUID NOT NULL,
    user1_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    user2_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_message_at TIMESTAMPTZ,
    last_message_preview TEXT,
    UNIQUE(user1_id, user2_id)
);

-- 添加唯一约束
ALTER TABLE conversations ADD CONSTRAINT conversations_conversation_id_key UNIQUE (conversation_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_conversations_user1_id ON conversations(user1_id);
CREATE INDEX IF NOT EXISTS idx_conversations_user2_id ON conversations(user2_id);
CREATE INDEX IF NOT EXISTS idx_conversations_last_message_at ON conversations(last_message_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE conversations ADD COLUMN id BIGSERIAL;
-- ALTER TABLE conversations DROP CONSTRAINT conversations_pkey CASCADE;
-- ALTER TABLE conversations ADD PRIMARY KEY (id);
-- ALTER TABLE conversations ADD CONSTRAINT conversations_conversation_id_key UNIQUE (conversation_id);

-- 修改用户ID字段类型的历史操作
-- ALTER TABLE conversations DROP CONSTRAINT IF EXISTS conversations_user1_id_fkey;
-- ALTER TABLE conversations DROP CONSTRAINT IF EXISTS conversations_user2_id_fkey;
-- ALTER TABLE conversations ADD COLUMN user1_uuid UUID;
-- ALTER TABLE conversations ADD COLUMN user2_uuid UUID;
-- UPDATE conversations c SET user1_uuid = u1.uuid, user2_uuid = u2.uuid FROM users u1, users u2 WHERE c.user1_id = u1.user_id AND c.user2_id = u2.user_id;
-- ALTER TABLE conversations ALTER COLUMN user1_uuid SET NOT NULL;
-- ALTER TABLE conversations ALTER COLUMN user2_uuid SET NOT NULL;
-- ALTER TABLE conversations DROP COLUMN user1_id;
-- ALTER TABLE conversations DROP COLUMN user2_id;
-- ALTER TABLE conversations RENAME COLUMN user1_uuid TO user1_id;
-- ALTER TABLE conversations RENAME COLUMN user2_uuid TO user2_id;
-- ALTER TABLE conversations ADD CONSTRAINT conversations_user1_id_fkey FOREIGN KEY (user1_id) REFERENCES users(uuid) ON DELETE CASCADE;
-- ALTER TABLE conversations ADD CONSTRAINT conversations_user2_id_fkey FOREIGN KEY (user2_id) REFERENCES users(uuid) ON DELETE CASCADE; 