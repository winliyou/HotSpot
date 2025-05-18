-- users表 - 存储用户基本信息
-- 文件：01_users.sql

-- 初始创建
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    login_id VARCHAR(100) NOT NULL UNIQUE,
    nickname VARCHAR(100) NOT NULL,
    password_hash TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 添加唯一约束
ALTER TABLE users ADD CONSTRAINT users_user_id_key UNIQUE (user_id);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS idx_users_last_active_at ON users(last_active_at);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
CREATE INDEX IF NOT EXISTS idx_users_updated_at ON users(updated_at);
CREATE INDEX IF NOT EXISTS idx_users_expires_at ON users(expires_at);

-- 历史修改记录
-- 以下是对表结构的历史修改，保留作为参考

-- 添加自增ID字段和修改主键
-- ALTER TABLE users ADD COLUMN id BIGSERIAL;
-- ALTER TABLE users DROP CONSTRAINT users_pkey CASCADE;
-- ALTER TABLE users ADD PRIMARY KEY (id);
-- ALTER TABLE users ADD CONSTRAINT users_user_id_key UNIQUE (user_id);

-- 临时用户说明：
-- 临时用户ID从99999999999开始，通过user_id >= 99999999999判断是否为临时用户

-- 创建自动清理过期临时用户的函数和触发器
CREATE OR REPLACE FUNCTION cleanup_expired_users()
RETURNS TRIGGER AS $$
BEGIN
    -- 删除过期的临时用户，临时用户ID从99999999999开始
    DELETE FROM users WHERE user_id >= 99999999999 AND expires_at < NOW();
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器以定期清理过期的临时用户数据
CREATE TRIGGER trigger_cleanup_expired_users
AFTER INSERT OR UPDATE ON users
FOR EACH STATEMENT EXECUTE FUNCTION cleanup_expired_users();