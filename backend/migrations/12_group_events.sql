-- group_events表 - 存储群组事件记录
-- 文件：12_group_events.sql

-- 初始创建群组事件表
CREATE TABLE IF NOT EXISTS group_events (
    id BIGSERIAL PRIMARY KEY,
    group_id UUID NOT NULL REFERENCES groups(group_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    target_user_id BIGINT REFERENCES users(user_id) ON DELETE CASCADE,
    event_type VARCHAR(10) NOT NULL CHECK (event_type IN ('create', 'join', 'leave', 'update', 'kick', 'transfer')),
    event_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_group_events_group_id ON group_events(group_id);
CREATE INDEX IF NOT EXISTS idx_group_events_user_id ON group_events(user_id);
CREATE INDEX IF NOT EXISTS idx_group_events_target_user_id ON group_events(target_user_id);
CREATE INDEX IF NOT EXISTS idx_group_events_event_type ON group_events(event_type);
CREATE INDEX IF NOT EXISTS idx_group_events_created_at ON group_events(created_at); 