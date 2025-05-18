---
description: 
globs: 
alwaysApply: true
---
# GeoTalk 项目需求文档

## 文档结构

- [GeoTalk 项目需求文档](#geotalk-项目需求文档)
  - [文档结构](#文档结构)
  - [1. 项目概览](#1-项目概览)
    - [1.1 项目简介](#11-项目简介)
    - [1.2 目标与愿景](#12-目标与愿景)
    - [1.3 核心功能概述](#13-核心功能概述)
    - [1.4 用户旅程与场景](#14-用户旅程与场景)
  - [2. 系统架构](#2-系统架构)
    - [2.1 总体架构](#21-总体架构)
      - [2.1.1 系统组件关系](#211-系统组件关系)
      - [2.1.2 数据流向](#212-数据流向)
    - [2.2 技术栈选择](#22-技术栈选择)
      - [2.2.1 前端技术栈](#221-前端技术栈)
      - [2.2.1.1 前端代码组织原则](#2211-前端代码组织原则)
      - [2.2.2 后端技术栈](#222-后端技术栈)
  - [3. 功能需求](#3-功能需求)
    - [3.1 用户管理功能](#31-用户管理功能)
      - [3.1.1 功能需求](#311-功能需求)
      - [3.1.2 API 接口](#312-api-接口)
    - [3.2 地理位置服务](#32-地理位置服务)
      - [3.2.1 功能需求](#321-功能需求)
      - [3.2.2 地图显示功能](#322-地图显示功能)
      - [3.2.3 地理位置计算实现](#323-地理位置计算实现)
      - [3.2.4 地理位置数据处理原则](#324-地理位置数据处理原则)
    - [3.3 社交功能](#33-社交功能)
      - [3.3.1 功能需求](#331-功能需求)
      - [3.3.2 API 接口](#332-api-接口)
    - [3.4 消息通讯系统](#34-消息通讯系统)
      - [3.4.1 功能需求](#341-功能需求)
      - [3.4.2 API 接口](#342-api-接口)
  - [5. 数据模型设计](#5-数据模型设计)
    - [5.1 用户数据模型](#51-用户数据模型)
      - [5.1.1 User表](#511-user表)
      - [5.1.2 UserLocation表](#512-userlocation表)
    - [5.2 群组数据模型](#52-群组数据模型)
      - [5.2.1 Group表](#521-group表)
      - [5.2.2 GroupMember表](#522-groupmember表)
    - [5.3 消息数据模型](#53-消息数据模型)
      - [5.3.1 GroupMessage表](#531-groupmessage表)
      - [5.3.2 Conversation表](#532-conversation表)
      - [5.3.3 DirectMessage表](#533-directmessage表)
    - [5.4 签到数据模型](#54-签到数据模型)
      - [5.4.1 Checkin表](#541-checkin表)
      - [5.4.2 Tag表](#542-tag表)
      - [5.4.3 CheckinTag表](#543-checkintag表)
      - [5.4.4 CheckinLike表](#544-checkinlike表)
    - [5.5 重要实现细节](#55-重要实现细节)
      - [5.5.1 地理位置实现](#551-地理位置实现)
      - [5.5.2 分页查询实现](#552-分页查询实现)
  - [6. API接口设计](#6-api接口设计)
    - [6.1 用户模块API](#61-用户模块api)
    - [6.2 群组模块API](#62-群组模块api)
    - [6.3 消息模块API](#63-消息模块api)
    - [6.4 签到模块API](#64-签到模块api)
  - [7. 改进建议](#7-改进建议)
    - [7.1 API字段命名一致性改进](#71-api字段命名一致性改进)
      - [统一使用creator前缀](#统一使用creator前缀)
      - [后端实现修改建议](#后端实现修改建议)

## 1. 项目概览

GeoTalk 是一个基于地理位置的社交应用，允许用户根据位置发现附近的用户、活动和群组，并进行实时通讯。本应用采用前后端分离架构，前端基于 Nuxt 3 + Vue 3 + Element Plus 开发，后端使用 Rust + Axum + PostgreSQL + Redis 实现。

### 1.1 项目简介
GeoTalk是一款专注于地理位置的社交应用，旨在连接同一区域的用户，促进线下社交互动。用户可以发现附近的人、创建和加入位置相关的群组，以及进行实时通讯。

### 1.2 目标与愿景
- 打造最便捷的基于位置的社交平台
- 促进线上社交向线下活动的转化
- 保护用户隐私的同时提供精确的位置服务
- 构建高性能、可扩展的实时通讯基础设施

### 1.3 核心功能概述
- 基于地理位置的用户发现
- 地理位置签到和动态分享
- 位置相关群组创建与管理
- 群组实时消息通讯
- 用户资料管理与登录注册

### 1.4 用户旅程与场景
- **场景一**：游客到达新城市，希望寻找当地活动和社交圈
- **场景二**：用户在特定地点(如会议中心)希望与其他参与者联系
- **场景三**：组织者创建基于位置的活动群组，吸引附近用户参与
- **场景四**：用户希望记录和分享自己去过的地方

## 2. 系统架构

### 2.1 总体架构

GeoTalk采用前后端分离的微服务架构设计，主要组件包括：

- 前端应用层：基于Nuxt 3构建的Web应用
- API网关层：处理请求路由、认证和负载均衡
- 微服务层：
  - 用户服务：处理用户注册、登录和资料管理
  - 位置服务：管理用户地理位置和邻近查询
  - 群组服务：管理群组创建、成员和设置
  - 消息服务：处理消息发送和存储
  - WebSocket服务：提供实时消息推送
- 数据存储层：
  - PostgreSQL+PostGIS：地理位置数据和业务数据持久化
  - Redis：会话缓存、地理信息索引和消息队列

#### 2.1.1 系统组件关系

```
客户端应用 <--> API网关 <--> 微服务集群(用户服务/消息服务/群组服务/位置服务)
    ^                             ^
    |                             |
    +---- WebSocket服务 <----------+
                ^
                |
          +-----+------+
          |            |
        Redis缓存    PostgreSQL
                      (PostGIS)
```

#### 2.1.2 数据流向

1. **位置更新流程**：
   - 用户位置更新 → 位置服务 → 更新数据库 → 更新Redis地理索引
   - 其他用户查询附近人 → 位置服务 → Redis地理索引 → 返回结果

2. **消息发送流程**：
   - 用户发送消息 → 消息服务 → 消息持久化 → WebSocket服务 → 推送给接收者
   - 离线用户消息 → 消息服务 → 存储未读消息 → 用户上线 → 推送离线消息

3. **群组互动流程**：
   - 创建群组 → 群组服务 → 位置服务 → 地理位置索引
   - 查找附近群组 → 位置服务 → 空间查询 → 群组服务 → 返回群组列表

### 2.2 技术栈选择

#### 2.2.1 前端技术栈
- 框架：Nuxt 3 + Vue 3 + TypeScript
- UI库：Element Plus
- 状态管理：Pinia
- 地图：高德地图

#### 2.2.1.1 前端代码组织原则
- **关注点分离**: 严格区分组件、页面和API调用职责
- **组件纯粹性**: 组件应只负责UI渲染和用户交互，不包含业务逻辑和API调用
- **页面职责**: 页面作为容器协调组件交互，负责数据获取和状态管理
- **API封装**: 所有后端接口调用应集中在独立的API服务层

#### 2.2.2 后端技术栈
- 语言：Rust（2024 edition）
- 框架：Axum 0.8.3
- 数据库：PostgreSQL（通过sqlx 0.8.4连接）
- 缓存：Redis 0.29.5
- 认证：JWT（使用jsonwebtoken 9.3.1）
- 日志：tracing + tracing-subscriber
- 其他重要库：
  - tokio（异步运行时）
  - chrono（时间处理）
  - uuid（唯一标识生成）
  - argon2（密码哈希）
  - serde/serde_json（序列化/反序列化）

## 3. 功能需求

### 3.1 用户管理功能

#### 3.1.1 功能需求

- 用户注册与登录
- 用户资料管理
- 基于位置查找附近用户

#### 3.1.2 API 接口

**注册接口**
```typescript
POST /api/v1/user/register
// 请求
interface RegisterRequest {
  login_id: string;   // 登录ID
  nickname: string;   // 用户昵称
  password: string;   // 用户密码
}
// 响应
interface AuthResponse {
  user_id: i64;       // 用户ID
  nickname: string;   // 用户昵称
  access_token: string;  // 访问令牌
  refresh_token: string; // 刷新令牌
  expires_at: string;    // 过期时间
}
```

**登录接口**
```typescript
POST /api/v1/user/login
// 请求
interface LoginRequest {
  login_id: string;   // 登录ID
  password: string;   // 密码
}
// 响应
interface AuthResponse {
  user_id: i64;       // 用户ID
  nickname: string;      // 用户昵称
  access_token: string;  // 访问令牌
  refresh_token: string; // 刷新令牌
  expires_at: string;    // 过期时间
}
```

**创建临时用户**
```typescript
POST /api/v1/user/create_temp_user
// 请求
interface CreateTempUserRequest {
  nickname?: string;  // 临时昵称，可选
}
// 响应
interface CreateTempUserResponse {
  user_id: i64;      // 临时用户ID
  nickname: string;     // 用户昵称
  access_token: string; // 访问令牌
  refresh_token: string; // 刷新令牌
  expires_at: string;   // 过期时间
}
```

**用户详情**
```typescript
POST /api/v1/user/search_by_id
// 请求
interface SearchUserByIdRequest {
  user_id: i64;  // 用户ID
}
// 响应
interface SearchUserByIdResponse {
  user_id: i64;      // 用户ID
  nickname: string;     // 用户昵称
  last_active: string;  // 最后活跃时间
  latitude: number;     // 纬度
  longitude: number;    // 经度
  distance: number;     // 距离当前用户的距离(米)
  location_name: string; // 位置名称
  online_status: string; // 在线状态
}
```

**附近用户查询**
```typescript
POST /api/v1/user/search_by_location
// 请求
interface NearbyUsersRequest {
  latitude: number;    // 纬度
  longitude: number;   // 经度
  radius: number;      // 搜索半径(米)，默认1000
  pagination: {
    limit?: number;      // 分页大小
    cursor?: number;     // 分页游标，可选
  }
}

interface NearbyUsersResponse {
  items: UserInfo[];      // 用户信息列表
  pagination: {
    has_more: boolean;        // 是否有更多结果
    next_cursor?: number;    // 下一页游标，可选
  }
}
```

**用户搜索**
```typescript
POST /api/v1/user/search_by_name
// 请求
interface SearchUserByNameRequest {
  keyword: string;    // 搜索关键词
  pagination: {
    limit?: number;     // 结果数量上限，可选
    cursor?: number;     // 分页游标，可选
  }
}
// 响应
interface SearchUserByNameResponse {
  items: UserInfo[];      // 用户信息列表
  pagination: {
    has_more: boolean;        // 是否有更多结果
    next_cursor?: number;     // 下一页游标，可选
  }
}

// UserInfo包含距离信息
interface UserInfo {
  user_id: i64;        // 用户ID
  nickname: string;       // 用户昵称
  last_active: string;    // 最后活跃时间
  latitude: number;       // 纬度
  longitude: number;      // 经度
  distance: number;       // 距离当前用户的实际距离(米)
  location_name: string;  // 位置名称
  online_status: string;  // 在线状态
}
```

**按ID搜索用户**
```typescript
POST /api/v1/user/search_by_id
// 请求
interface SearchUserByIdRequest {
  user_id: i64;      // 用户ID
}
// 响应
interface SearchUserByIdResponse {
  // 继承自UserInfo，包含所有UserInfo字段，包括distance(实际距离)
}
```

**更新位置**
```typescript
POST /api/v1/user/update_location
// 请求
interface UpdateLocationRequest {
  latitude: number;      // 纬度，客户端无法获取时使用默认值112.68
  longitude: number;     // 经度，客户端无法获取时使用默认值35.16
  location_name: string; // 位置名称，客户端无法获取时使用默认值"南天门"
}
// 响应
// 返回空数据
```

### 3.2 地理位置服务

#### 3.2.1 功能需求

- 获取与更新用户当前位置
- 地理位置签到与展示
- 根据坐标获取地址名称
- 地图显示与交互
- 当无法获取用户位置时，使用默认位置（经度112.68,纬度35.16,"南天门"）

#### 3.2.2 地图显示功能

- 显示用户当前位置
- 显示附近的活动、用户和群组
- 支持地图缩放和移动
- 使用高德地图实现地图交互

#### 3.2.3 地理位置计算实现

- 使用PostgreSQL的PostGIS扩展的`ST_Distance`函数计算地理位置之间的真实距离
- 所有涉及距离搜索的API在响应中包含精确距离信息（以米为单位）
- 确保查询SQL中使用正确的参数绑定顺序
- 当用户没有位置数据时，使用系统默认位置值（DEFAULT_LATITUDE=35.16, DEFAULT_LONGITUDE=112.68, DEFAULT_LOCATION_NAME="南天门"）
- 在查询中使用COALESCE处理空值，确保即使某些位置信息缺失，距离计算也能进行
- 前端显示准确的距离信息，自动转换为合适的单位（米/公里）
- 按地理距离排序搜索结果
- 确保排序和筛选基于真实地理距离

SQL查询示例（以搜索用户为例）：
```sql
WITH current_user_loc AS (
    SELECT longitude, latitude 
    FROM user_locations 
    WHERE user_id = $1
)
SELECT u.id, u.user_id, u.nickname, u.last_active_at, 
CASE WHEN u.last_active_at > NOW() - INTERVAL '10 minutes' THEN 'online' 
     WHEN u.last_active_at > NOW() - INTERVAL '1 hour' THEN 'away' 
     ELSE 'offline' 
END as online_status, 
COALESCE(ul.latitude, $3) as latitude,
COALESCE(ul.longitude, $4) as longitude,
COALESCE(ul.location_name, $5) as location_name,
ST_Distance(
    ST_MakePoint(COALESCE(ul.longitude, $4), COALESCE(ul.latitude, $3))::geography,
    ST_MakePoint(COALESCE(c.longitude, $4), COALESCE(c.latitude, $3))::geography
) as distance
FROM users u
LEFT JOIN user_locations ul ON u.user_id = ul.user_id
LEFT JOIN current_user_loc c ON true
WHERE u.nickname ILIKE $2
AND u.user_id != $1
```

参数绑定顺序规范：
- 首先绑定所有常规参数（用户ID、搜索条件等）
- 其次绑定默认位置相关参数（DEFAULT_LATITUDE、DEFAULT_LONGITUDE等）

#### 3.2.4 地理位置数据处理原则

- **数据收集**：后端应尽可能收集并存储详细的地理位置数据(经纬度、时间戳、精度)
- **数据存储**：使用专用的地理数据类型和空间索引优化存储和查询
- **数据分析**：详细的位置数据将用于后期热点分析、用户轨迹研究和推荐算法
- **前端展示**：API响应中仅包含前端展示必要的位置信息(如距离、简化坐标)
- **隐私保护**：实现多级精度的位置显示，允许用户设置位置信息的可见范围
- **默认位置处理**：当客户端无法获取用户坐标和位置名称时，使用默认坐标(经度112.68,纬度35.16)和默认位置名称"南天门"

### 3.3 社交功能

#### 3.3.1 功能需求

- 创建基于位置的群组
- 设置群组名称、描述和可选密码
- 管理群组成员角色(管理员、普通成员)
- 按位置搜索附近群组
- 加入与退出群组

#### 3.3.2 API 接口

**创建群组**
```typescript
POST /api/v1/group/create
// 请求
interface CreateGroupRequest {
  name: string;           // 群组名称
  description?: string;   // 群组描述，可选
  location_name: string;  // 位置名称，客户端无法获取时使用默认值"南天门"
  latitude: number;       // 纬度，客户端无法获取时使用默认值35.16
  longitude: number;      // 经度，客户端无法获取时使用默认值112.68
  password?: string;      // 可选密码
}
// 响应
interface CreateGroupResponse {
  group_id: string;       // 群组ID
}
```

**获取群组信息**
```typescript
POST /api/v1/group/search_by_id
// 请求
interface SearchGroupByIdRequest {
  group_id: string;       // 群组ID
}
// 响应
interface GroupInfo {
  group_id: string;                // 群组ID
  name: string;                    // 群组名称
  description?: string;             // 群组描述，可选
  creator_id: i64;                // 创建者ID
  creator_name: string;            // 创建者名称
  created_at: string;              // 创建时间
  last_active_at: string;          // 最后活跃时间
  latitude: number;                // 纬度
  longitude: number;               // 经度
  member_count: number;            // 成员数量
  distance: number;                // 距离(米)
  location_name: string;           // 位置名称
  is_password_required: boolean;   // 是否需要密码
  is_member: boolean;              // 当前用户是否为成员
  user_role: string;               // 当前用户的角色
}
```

**群组成员管理**
```typescript
POST /api/v1/group/join   // 加入群组
// 请求
interface JoinGroupRequest {
  group_id: string;   // 群组ID
  password?: string;  // 群组密码，如需
}
// 响应
interface JoinGroupResponse {
  success: boolean;    // 是否成功加入
  role: string;        // 加入后的角色，如"member"
}

POST /api/v1/group/leave  // 离开群组
// 请求
interface LeaveGroupRequest {
  group_id: string;  // 群组ID
}
// 响应
interface LeaveGroupResponse {
  success: boolean;  // 是否成功离开
}

POST /api/v1/group/members   // 获取群组成员
// 请求
interface GroupMembersRequest {
  group_id: string;  // 群组ID
  cursor?: number;   // 分页游标，可选
  limit?: number;    // 结果数量上限，可选
}
// 响应
interface GroupMembersResponse {
  members: {
    user_id: i64;      // 用户ID
    nickname: string;     // 用户昵称
    last_active: string;  // 最后活跃时间
    role: string;         // 成员角色，如"admin"或"member"
    join_time: string;    // 加入时间
  }[];
  has_more: boolean;      // 是否有更多数据
  next_cursor?: number;   // 下一页游标，可选
}
```

**获取附近群组**
```typescript
POST /api/v1/group/search_by_location
// 请求
interface SearchGroupByLocationRequest {
  latitude: number;     // 纬度
  longitude: number;    // 经度
  radius: number;       // 半径(米)
  cursor?: number;      // 分页游标，可选
  limit?: number;       // 结果数量上限，可选
}
// 响应
interface SearchGroupByLocationResponse {
  groups: GroupInfo[];  // 群组信息列表
  has_more: boolean;    // 是否有更多数据
  next_cursor?: number; // 下一页游标，可选
}
```

**搜索群组接口**
```typescript
POST /api/v1/group/search_by_name
// 请求
interface SearchGroupByNameRequest {
  keyword: string;     // 搜索关键词(群组名称)
  cursor?: number;     // 分页游标，可选
  limit?: number;      // 结果数量上限，可选
}
// 响应
interface SearchGroupByNameResponse {
  groups: GroupInfo[];  // 群组信息列表
  has_more: boolean;    // 是否有更多数据
  next_cursor?: number; // 下一页游标，可选
}
```

### 3.4 消息通讯系统

#### 3.4.1 功能需求

- 发送和接收实时消息
- 支持表情和链接识别
- 查看历史消息
- 删除自己发送的消息
- 图片消息功能将在后续版本中支持
- 用户间私聊交流

#### 3.4.2 API 接口

**发送群组消息**
```typescript
POST /api/v1/chat/group/send
// 请求
interface SendGroupMessageRequest {
  group_id: string;     // 群组ID
  content: string;       // 消息内容
  message_type: string;  // 消息类型，默认为"text"
  latitude: number;      // 纬度
  longitude: number;     // 经度
}
// 响应
interface SendGroupMessageResponse {
  message_id: string;     // 消息ID
  group_id: string;       // 群组ID 
  message_type: string;   // 消息类型
  sent_at: string;        // 发送时间
}
```

**发送私聊消息**
```typescript
POST /api/v1/chat/user/send
// 请求
interface SendDirectMessageRequest {
  recipient_id: i64;  // 接收者ID
  content: string;       // 消息内容
  message_type: string;  // 消息类型，默认为"text"
  latitude: number;      // 纬度，可选，客户端无法获取时使用默认值35.16
  longitude: number;     // 经度，可选，客户端无法获取时使用默认值112.68
}
// 响应
interface SendDirectMessageResponse {
  message_id: string;     // 消息ID
  conversation_id: string; // 会话ID
  recipient_id: i64;    // 接收者ID
  message_type: string;    // 消息类型
  sent_at: string;         // 发送时间
}
```

**获取私聊消息历史**
```typescript
POST /api/v1/chat/user/history
// 请求
interface DirectMessageHistoryRequest {
  conversation_id?: string; // 会话ID，可选
  user_id?: i64;         // 用户ID，可选(与conversation_id二选一)
  limit?: number;           // 每页条数，可选
  cursor?: number;          // 分页游标，可选
}
// 响应
interface DirectMessageHistoryResponse {
  messages: {
    message_id: string;         // 消息ID
    conversation_id: string;    // 会话ID
    sender_id: i64;          // 发送者ID
    sender_name: string;        // 发送者名称
    recipient_id: i64;       // 接收者ID
    recipient_name: string;     // 接收者名称
    content: string;            // 消息内容
    message_type: string;       // 消息类型
    sent_at: string;            // 发送时间
    read_at: string | null;     // 已读时间，可为null
    latitude: number;           // 纬度
    longitude: number;          // 经度
  }[];
  has_more: boolean;            // 是否有更多消息
  next_cursor?: number;         // 下一页游标，可选
}
```

**获取私聊会话列表**
```typescript
POST /api/v1/chat/user/conversations
// 请求
interface ConversationsRequest {
  limit?: number;           // 每页条数，可选
  cursor?: number;          // 分页游标，可选
}
// 响应
interface ConversationsResponse {
  conversations: {
    conversation_id: string;  // 会话ID
    peer_id: i64;         // 对方用户ID
    peer_name: string;       // 对方用户名称
    last_message: string;    // 最后一条消息内容
    last_message_time: string; // 最后一条消息时间
    unread_count: number;    // 未读消息数量
  }[];
  has_more: boolean;         // 是否有更多会话
  next_cursor?: number;      // 下一页游标，可选
}
```

**标记私聊消息已读**
```typescript
POST /api/v1/chat/user/mark-read
// 请求
interface MarkReadRequest {
  conversation_id: string;  // 会话ID
  message_id?: string;      // 消息ID，可选(不提供时标记所有消息为已读)
}
// 响应
interface MarkReadResponse {
  success: boolean;         // 是否成功
  marked_count?: number;    // 已标记数量，可选
}
```

**删除私聊消息**
```typescript
POST /api/v1/chat/user/delete
// 请求
interface DeleteDirectMessageRequest {
  message_id: string;       // 消息ID
}
// 响应
interface DeleteMessageResponse {
  success: boolean;         // 是否成功
}
```

**获取群组消息历史**
```typescript
POST /api/v1/chat/group/history
// 请求
interface GroupMessageHistoryRequest {
  group_id: string;     // 群组ID
  cursor?: number;      // 分页游标，可选
  limit?: number;       // 每页条数，可选
}
// 响应
interface GroupMessageHistoryResponse {
  messages: {
    message_id: string;         // 消息ID
    group_id: string;           // 群组ID
    sender_id: i64;          // 发送者ID
    sender_name: string;        // 发送者名称
    content: string;            // 消息内容
    message_type: string;       // 消息类型
    sent_at: string;            // 发送时间
    latitude: number;           // 纬度
    longitude: number;          // 经度
  }[];
  has_more: boolean;            // 是否有更多消息
  next_cursor?: number;         // 下一页游标，可选
}
```

- 认证失败处理：
  - 令牌无效或过期时，服务器发送认证失败消息并关闭连接
  - 客户端应监听此消息并尝试通过刷新API获取新令牌，然后重新连接

## 5. 数据模型设计

### 5.1 用户数据模型

#### 5.1.1 User表

| 字段名         | 类型         | 说明                | 索引     |
| -------------- | ------------ | ------------------- | -------- |
| id             | BIGSERIAL    | 自增主键            | 主键索引 |
| user_id        | BIGINT       | 对外暴露的用户标识  | 唯一索引 |
| login_id       | VARCHAR(100) | 登录标识(邮箱/手机) | 唯一索引 |
| nickname       | VARCHAR(100) | 用户昵称            | -        |
| password_hash  | TEXT         | 密码哈希            | -        |
| created_at     | TIMESTAMPTZ  | 创建时间            | 索引     |
| updated_at     | TIMESTAMPTZ  | 更新时间            | 索引     |
| last_active_at | TIMESTAMPTZ  | 最后活跃时间        | 索引     |
| is_temp        | BOOLEAN      | 是否为临时用户      | 索引     |

#### 5.1.2 UserLocation表

| 字段名        | 类型             | 说明         | 索引     |
| ------------- | ---------------- | ------------ | -------- |
| id            | BIGSERIAL        | 自增主键     | 主键索引 |
| user_id       | BIGINT           | 用户ID(外键) | 索引     |
| latitude      | DOUBLE PRECISION | 纬度         | -        |
| longitude     | DOUBLE PRECISION | 经度         | -        |
| updated_at    | TIMESTAMPTZ      | 更新时间     | 索引     |
| location_name | TEXT             | 位置名称     | -        |
| geom          | GEOGRAPHY(POINT) | 地理点       | 空间索引 |

### 5.2 群组数据模型

#### 5.2.1 Group表

| 字段名         | 类型             | 说明               | 索引     |
| -------------- | ---------------- | ------------------ | -------- |
| id             | BIGSERIAL        | 自增主键           | 主键索引 |
| group_id       | UUID             | 对外暴露的群组标识 | 唯一索引 |
| name           | VARCHAR(100)     | 群组名称           | 索引     |
| description    | TEXT             | 群组描述           | -        |
| creator_id     | BIGINT           | 创建者ID           | 索引     |
| created_at     | TIMESTAMPTZ      | 创建时间           | 索引     |
| updated_at     | TIMESTAMPTZ      | 更新时间           | 索引     |
| last_active_at | TIMESTAMPTZ      | 最后活跃时间       | 索引     |
| latitude       | DOUBLE PRECISION | 纬度               | -        |
| longitude      | DOUBLE PRECISION | 经度               | -        |
| location_name  | TEXT             | 位置名称           | -        |
| password_hash  | TEXT             | 密码哈希（可选）   | -        |
| geom           | GEOGRAPHY(POINT) | 地理点             | 空间索引 |

#### 5.2.2 GroupMember表

| 字段名         | 类型        | 说明                             | 索引     |
| -------------- | ----------- | -------------------------------- | -------- |
| id             | BIGSERIAL   | 自增主键                         | 主键索引 |
| group_id       | UUID        | 群组ID(外键)                     | 索引     |
| user_id        | BIGINT      | 用户ID(外键)                     | 索引     |
| role           | VARCHAR(10) | 成员角色(admin/owner/member)     | 索引     |
| joined_at      | TIMESTAMPTZ | 加入时间                         | 索引     |

### 5.3 消息数据模型

#### 5.3.1 GroupMessage表

| 字段名       | 类型             | 说明                        | 索引     |
| ------------ | ---------------- | --------------------------- | -------- |
| id           | BIGSERIAL        | 自增主键                    | 主键索引 |
| message_id   | UUID             | 消息唯一标识                | 唯一索引 |
| group_id     | UUID             | 群组ID(外键)                | 索引     |
| sender_id    | BIGINT           | 发送者ID(外键)              | 索引     |
| content      | TEXT             | 消息内容                    | -        |
| message_type | VARCHAR(20)      | 消息类型(text/image/system) | 索引     |
| created_at   | TIMESTAMPTZ      | 发送时间                    | 索引     |
| latitude     | DOUBLE PRECISION | 纬度                        | -        |
| longitude    | DOUBLE PRECISION | 经度                        | -        |

#### 5.3.2 Conversation表

| 字段名               | 类型        | 说明             | 索引                         |
| -------------------- | ----------- | ---------------- | ---------------------------- |
| id                   | BIGSERIAL   | 自增主键         | 主键索引                     |
| conversation_id      | UUID        | 会话唯一标识     | 唯一索引                     |
| user1_id             | BIGINT      | 用户1ID(外键)    | 复合索引(user1_id, user2_id) |
| user2_id             | BIGINT      | 用户2ID(外键)    | 复合索引(user1_id, user2_id) |
| created_at           | TIMESTAMPTZ | 创建时间         | 索引                         |
| updated_at           | TIMESTAMPTZ | 更新时间         | 索引                         |
| last_message_at      | TIMESTAMPTZ | 最后一条消息时间 | 索引                         |
| last_message_preview | TEXT        | 最后一条消息预览 | -                            |

#### 5.3.3 DirectMessage表

| 字段名          | 类型             | 说明                        | 索引     |
| --------------- | ---------------- | --------------------------- | -------- |
| id              | BIGSERIAL        | 自增主键                    | 主键索引 |
| message_id      | UUID             | 消息唯一标识                | 唯一索引 |
| conversation_id | UUID             | 会话ID(外键)                | 索引     |
| sender_id       | BIGINT           | 发送者ID(外键)              | 索引     |
| recipient_id    | BIGINT           | 接收者ID(外键)              | 索引     |
| content         | TEXT             | 消息内容                    | -        |
| message_type    | VARCHAR(20)      | 消息类型(text/image/system) | 索引     |
| created_at      | TIMESTAMPTZ      | 发送时间                    | 索引     |
| read_at         | TIMESTAMPTZ      | 已读时间                    | 索引     |
| latitude        | DOUBLE PRECISION | 纬度                        | -        |
| longitude       | DOUBLE PRECISION | 经度                        | -        |

### 5.4 签到数据模型

#### 5.4.1 Checkin表

| 字段名        | 类型             | 说明           | 索引     |
| ------------- | ---------------- | -------------- | -------- |
| id            | BIGSERIAL        | 自增主键       | 主键索引 |
| checkin_id    | UUID             | 签到唯一标识符 | 唯一索引 |
| user_id       | BIGINT           | 用户ID(外键)   | 索引     |
| description   | TEXT             | 签到描述       | -        |
| latitude      | DOUBLE PRECISION | 纬度           | -        |
| longitude     | DOUBLE PRECISION | 经度           | -        |
| location_name | TEXT             | 位置名称       | -        |
| created_at    | TIMESTAMPTZ      | 创建时间       | 索引     |
| geom          | GEOGRAPHY(POINT) | 地理点         | 空间索引 |

#### 5.4.2 Tag表

| 字段名 | 类型        | 说明     | 索引     |
| ------ | ----------- | -------- | -------- |
| id     | BIGSERIAL   | 自增主键 | 主键索引 |
| name   | VARCHAR(50) | 标签名称 | 唯一索引 |

#### 5.4.3 CheckinTag表

| 字段名     | 类型      | 说明         | 索引     |
| ---------- | --------- | ------------ | -------- |
| id         | BIGSERIAL | 自增主键     | 主键索引 |
| checkin_id | UUID      | 签到ID(外键) | 索引     |
| tag_id     | BIGINT    | 标签ID(外键) | 索引     |

#### 5.4.4 CheckinLike表

| 字段名     | 类型        | 说明         | 索引     |
| ---------- | ----------- | ------------ | -------- |
| id         | BIGSERIAL   | 自增主键     | 主键索引 |
| checkin_id | UUID        | 签到ID(外键) | 索引     |
| user_id    | BIGINT      | 用户ID(外键) | 索引     |
| created_at | TIMESTAMPTZ | 创建时间     | 索引     |

### 5.5 重要实现细节

#### 5.5.1 地理位置实现

- 所有涉及地理位置的表使用PostGIS扩展
- 使用`geom`字段存储地理点数据，类型为`GEOGRAPHY(POINT, 4326)`
- 创建自动更新触发器，当经纬度更新时自动更新地理点字段：

```sql
CREATE OR REPLACE FUNCTION update_geom_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.geom = ST_SetSRID(ST_MakePoint(NEW.longitude, NEW.latitude), 4326)::geography;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_locations_geom
BEFORE INSERT OR UPDATE OF latitude, longitude ON user_locations
FOR EACH ROW EXECUTE FUNCTION update_geom_column();
```

- 地理位置查询使用`ST_Distance`函数计算距离，以米为单位
- 使用GiST索引优化地理位置查询性能

#### 5.5.2 分页查询实现

- 使用基于游标的分页方式，而非偏移量分页
- 游标必须始终与表的 id 字段进行比较，而不是与 user_id、group_id（UUID）或其他业务标识字段比较，禁止将UUID等主键转为text后做分页比较
- 标准查询模式为：`WHERE id < $cursor ORDER BY id DESC LIMIT $limit + 1`
- 查询比请求的limit多一条记录，用于判断是否还有更多数据
- 下一页游标值应使用最后一条记录的 id 字段值，而不是使用其他字段
- 分页响应结构：

```typescript
interface PaginatedResponse<T> {
  items: T[];               // 数据项列表
  pagination: {
    has_more: boolean;      // 是否有更多数据
    next_cursor?: number;   // 下一页游标，有更多数据时提供
  }
}
```

- 游标分页在返回较大结果集时比偏移量分页更高效，因为数据库不需要跳过大量记录
- 按时间顺序或其他非 id 字段排序时，ORDER BY 子句应包含 id 作为次要排序条件，确保分页结果唯一且稳定
- **所有分页API必须用自增主键id（i64）做游标，禁止用UUID或text类型游标。**
- **群组相关分页（如search_group_by_location、search_group_by_name）必须用groups表的id字段做游标分页，不能用group_id（UUID）或其text形式。**
- 违例示例：`WHERE group_id::text > $cursor::text`
- 正确示例：`WHERE id < $cursor`
- 该规范适用于所有分页API，包括用户、群组、消息、签到等。

## 6. API接口设计

### 6.1 用户模块API

| 功能           | 路径                              | 方法 | 描述                   |
| -------------- | --------------------------------- | ---- | ---------------------- |
| 用户注册       | `/api/v1/user/register`           | POST | 注册新用户             |
| 用户登录       | `/api/v1/user/login`              | POST | 用户登录并获取令牌     |
| 创建临时用户   | `/api/v1/user/create_temp_user`   | POST | 创建临时访客用户       |
| 按名称搜索用户 | `/api/v1/user/search_by_name`     | POST | 根据用户名称搜索用户   |
| 按ID搜索用户   | `/api/v1/user/search_by_id`       | POST | 根据用户ID搜索用户     |
| 按位置搜索用户 | `/api/v1/user/search_by_location` | POST | 搜索指定位置附近的用户 |
| 更新用户位置   | `/api/v1/user/update_location`    | POST | 更新用户当前地理位置   |
| 刷新令牌       | `/api/v1/user/refresh_token`      | POST | 刷新用户的访问令牌     |

### 6.2 群组模块API

| 功能           | 路径                               | 方法 | 描述               |
| -------------- | ---------------------------------- | ---- | ------------------ |
| 创建群组       | `/api/v1/group/create`             | POST | 创建新群组         |
| 按位置搜索群组 | `/api/v1/group/search_by_location` | POST | 按地理位置搜索群组 |
| 按名称搜索群组 | `/api/v1/group/search_by_name`     | POST | 按名称搜索群组     |
| 按ID搜索群组   | `/api/v1/group/search_by_id`       | POST | 按ID搜索群组       |
| 加入群组       | `/api/v1/group/join`               | POST | 加入指定群组       |
| 离开群组       | `/api/v1/group/leave`              | POST | 离开指定群组       |
| 获取群组成员   | `/api/v1/group/members`            | POST | 获取群组成员列表   |

### 6.3 消息模块API

| 功能             | 路径                              | 方法 | 描述                 |
| ---------------- | --------------------------------- | ---- | -------------------- |
| 发送群组消息     | `/api/v1/chat/group/send`         | POST | 发送消息到群组       |
| 获取群组消息历史 | `/api/v1/chat/group/history`      | POST | 获取群组聊天历史     |
| 发送私聊消息     | `/api/v1/chat/user/send`          | POST | 发送私聊消息给用户   |
| 获取私聊消息历史 | `/api/v1/chat/user/history`       | POST | 获取私聊消息历史     |
| 获取会话列表     | `/api/v1/chat/user/conversations` | POST | 获取用户所有会话列表 |
| 删除私聊消息     | `/api/v1/chat/user/delete`        | POST | 删除私聊消息         |
| 标记消息已读     | `/api/v1/chat/user/mark-read`     | POST | 标记消息为已读状态   |

### 6.4 签到模块API

| 功能           | 路径                                 | 方法 | 描述                   |
| -------------- | ------------------------------------ | ---- | ---------------------- |
| 创建签到       | `/api/v1/checkin/create`             | POST | 创建新的位置签到       |
| 删除签到       | `/api/v1/checkin/delete`             | POST | 删除已创建的签到       |
| 获取签到历史   | `/api/v1/checkin/history`            | POST | 获取用户签到历史记录   |
| 按位置搜索签到 | `/api/v1/checkin/search_by_location` | POST | 搜索指定位置附近的签到 |
| 按标签搜索签到 | `/api/v1/checkin/search_by_tags`     | POST | 按标签搜索签到         |
| 按ID搜索签到   | `/api/v1/checkin/search_by_id`       | POST | 按ID搜索特定签到       |
| 点赞签到       | `/api/v1/checkin/like`               | POST | 对签到点赞             |
| 取消点赞       | `/api/v1/checkin/unlike`             | POST | 取消对签到的点赞       |

## 7. 改进建议

### 7.1 API字段命名一致性改进

在文档中，我们统一使用 `creator_id` 和 `creator_name` 字段表示群组创建者，确保命名的一致性。在实际后端实现中，如果仍使用 `owner_id`，建议进行以下修改以统一命名：

#### 统一使用creator前缀

```typescript
interface GroupInfo {
  // ...其他字段保持不变
  creator_id: i64;          // 创建者ID
  creator_name: string;     // 创建者名称
  // ...
}
```

#### 后端实现修改建议

为了统一使用creator前缀，后端需要进行以下修改：

1. 修改数据库表结构：

```sql
-- 修改列名
ALTER TABLE groups RENAME COLUMN owner_id TO creator_id;

-- 更新索引
DROP INDEX IF EXISTS idx_groups_owner_id;
CREATE INDEX idx_groups_creator_id ON groups(creator_id);
```

2. 修改`GroupRow`和`GroupInfo`结构体：

```rust
// 修改前
pub struct GroupRow {
    // ...其他字段
    pub owner_id: i64,
    pub creator_name: String,
    // ...
}

// 修改后
pub struct GroupRow {
    // ...其他字段
    pub creator_id: i64,  // 重命名字段
    pub creator_name: String,
    // ...
}
```

3. 修改相关SQL查询:

```rust
// 修改前
sqlx::query_as!(
    GroupRow,
    r#"
    SELECT 
        // ...其他字段
        g.owner_id as "owner_id!",
        u.nickname as "creator_name!",
        // ...
    FROM groups g
    JOIN users u ON g.owner_id = u.user_id
    // ...
    "#,
    // ...
)

// 修改后
sqlx::query_as!(
    GroupRow,
    r#"
    SELECT 
        // ...其他字段
        g.creator_id as "creator_id!",  // 修改字段名
        u.nickname as "creator_name!",
        // ...
    FROM groups g
    JOIN users u ON g.creator_id = u.user_id  // 修改关联条件
    // ...
    "#,
    // ...
)
```

4. 更新controller中的映射代码:

```rust
// 修改前
let info = GroupInfo {
    // ...其他字段
    owner_id: result.owner_id,
    creator_name: result.creator_name,
    // ...
};

// 修改后
let info = GroupInfo {
    // ...其他字段
    creator_id: result.creator_id,  // 使用新字段名
    creator_name: result.creator_name,
    // ...
};
```

这种改进将确保API的字段命名保持一致，提高代码的可读性和可维护性。为了避免破坏现有数据和功能，建议对此更改进行完整的单元测试和集成测试。