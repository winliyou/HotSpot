-- PostgreSQL扩展 - 用于地理位置功能
-- 文件：00_extensions.sql

-- 启用postgis扩展以支持地理位置查询
CREATE EXTENSION IF NOT EXISTS postgis CASCADE; 

-- 启用uuid-ossp扩展以支持UUID生成
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";