# GeoTalk - 基于地理位置的社交应用

GeoTalk是一款专注于地理位置的社交应用，旨在连接同一区域的用户，促进线下社交互动。用户可以发现附近的人、创建和加入位置相关的群组，以及进行实时通讯。

## 项目架构

本项目采用前后端分离架构：

- 前端：Nuxt 3 + Vue 3 + Naive UI
- 后端：Rust + Axum + PostgreSQL + Redis

## 后端功能

- 用户管理（注册、登录、临时用户）
- 地理位置服务（位置更新、附近用户查询）
- 群组功能（创建群组、加入群组、群组消息）
- 实时消息通讯（WebSocket消息推送）
- 位置签到与分享

## 环境要求

- Rust 1.75+
- PostgreSQL 15+ (with PostGIS extension)
- Redis 7+

## 开发环境设置

1. 克隆项目
```bash
git clone https://github.com/yourusername/geotalk.git
cd geotalk
```

2. 设置环境变量
```bash
cp .env.example .env
# 编辑.env文件，配置数据库连接信息
```

3. 安装PostgreSQL扩展
```sql
CREATE EXTENSION postgis;
CREATE EXTENSION earthdistance;
CREATE EXTENSION cube;
```

4. 创建数据库
```bash
createdb geotalk
psql -d geotalk -f src/scripts/create_tables.sql
```

5. 运行后端
```bash
cargo run
```

## API文档

### 用户模块

- POST /api/user/register - 用户注册
- POST /api/user/login - 用户登录
- POST /api/user/temp-create - 创建临时用户
- POST /api/user/nearby - 查询附近用户
- POST /api/user/search - 搜索用户
- POST /api/user/detail - 获取用户详情
- POST /api/user/update-location - 更新用户位置

### WebSocket

- GET /ws - WebSocket连接

## 技术栈详情

- **Axum**: Web框架
- **SQLx**: 异步SQL工具包
- **Redis**: 用于会话管理和GEO查询
- **PostgreSQL/PostGIS**: 地理位置数据存储与查询
- **JSON Web Token (JWT)**: 用户认证
- **WebSocket**: 实时消息推送 