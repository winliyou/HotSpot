# 数据库表结构说明

本目录包含了按表分开的SQL文件，便于理解数据库结构和修改表结构。

## 文件命名规则

文件按照创建顺序命名，确保依赖关系正确处理：

- `00_extensions.sql`: 数据库扩展
- `01_users.sql`: 用户表
- `02_user_locations.sql`: 用户位置表
- `03_groups.sql`: 群组表
- `04_group_members.sql`: 群组成员表
- `05_group_messages.sql`: 群组消息表
- `06_conversations.sql`: 会话表(私聊)
- `07_direct_messages.sql`: 直接消息表(私聊消息)
- `08_tags.sql`: 标签表
- `09_checkins.sql`: 签到表
- `10_checkin_tags.sql`: 签到标签关联表
- `11_checkin_likes.sql`: 签到点赞表

## 表结构关系

- `users`: 核心表，存储用户基本信息
- `user_locations`: 每个用户的位置信息
- `groups`: 群组信息
- `group_members`: 群组成员关系
- `group_messages`: 群组内的消息
- `conversations`: 用户间一对一会话
- `direct_messages`: 一对一私聊消息
- `tags`: 可用于签到的标签
- `checkins`: 用户位置签到
- `checkin_tags`: 签到与标签的多对多关系
- `checkin_likes`: 签到的点赞记录

## 修改历史记录

SQL文件中已注释的ALTER语句为历史上对表结构的修改记录，保留以供参考，实际使用中已注释掉，这些语句主要记录:

1. 用户表UUID字段的添加与删除
2. 关系表中用户ID字段类型从BIGINT到UUID的转换 