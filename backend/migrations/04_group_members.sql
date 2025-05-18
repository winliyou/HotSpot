-- group_members表 - 存储群组成员关系
-- 文件：04_group_members.sql

-- 先确保枚举类型已存在
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'group_role') THEN
        CREATE TYPE group_role AS ENUM ('admin', 'owner', 'member');
    END IF;
END
$$;

-- 初始创建
CREATE TABLE IF NOT EXISTS group_members (
    id BIGSERIAL PRIMARY KEY,
    group_id UUID NOT NULL REFERENCES groups(group_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    role VARCHAR(10) NOT NULL DEFAULT 'member' CHECK (role IN ('admin', 'owner', 'member')), -- 使用VARCHAR替代枚举
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 添加唯一约束，确保用户在一个群组中只有一个记录
ALTER TABLE group_members ADD CONSTRAINT group_members_group_user_unique UNIQUE (group_id, user_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_group_members_group_id ON group_members(group_id);
CREATE INDEX IF NOT EXISTS idx_group_members_user_id ON group_members(user_id);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE group_members ADD COLUMN id BIGSERIAL;
-- ALTER TABLE group_members DROP CONSTRAINT group_members_pkey;
-- ALTER TABLE group_members ADD PRIMARY KEY (id);
-- ALTER TABLE group_members ADD CONSTRAINT group_members_group_user_unique UNIQUE (group_id, user_id);

-- 修改用户ID字段类型的历史操作
-- ALTER TABLE group_members DROP CONSTRAINT IF EXISTS group_members_user_id_fkey;
-- ALTER TABLE group_members ADD COLUMN user_uuid UUID;
-- UPDATE group_members gm SET user_uuid = u.uuid FROM users u WHERE gm.user_id = u.user_id;
-- ALTER TABLE group_members ALTER COLUMN user_uuid SET NOT NULL;
-- ALTER TABLE group_members DROP COLUMN user_id;
-- ALTER TABLE group_members RENAME COLUMN user_uuid TO user_id;
-- ALTER TABLE group_members ADD CONSTRAINT group_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(uuid) ON DELETE CASCADE; 