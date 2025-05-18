---
applyTo: '**'
---
# HotSpot 项目开发规范

## 后端规范

### 依赖管理规则
- 不能直接修改 Cargo.toml 文件
- 使用 `cargo add xxx --features yyy,zzz` 增加依赖
- 使用 `cargo remove xxx` 删除依赖
- 引入新依赖前，需联网查询该依赖包的最新版本、稳定性、维护状态和可能的替代方案
- 评估依赖安全性和兼容性，避免使用已弃用包

### 代码结构规则
- 后端代码在 `backend` 目录下
- 数据库表定义在 `migrations` 目录下
- 严格分层架构设计
- 生成/修改文件时需使用全路径，确保在正确目录下

### 分层架构

#### Controller 层
- 处理 HTTP 请求参数解析和校验
- 调用 Service 层方法完成业务逻辑
- 将 Service 层返回的领域模型转换为 API 响应格式
- 处理错误并返回适当的 HTTP 响应

#### Service 层
- 实现业务逻辑和规则
- 直接与数据库交互
- 返回领域模型或工具模型，不返回 HTTP 相关响应格式
- 遵循原子化原则，每个函数实现最小化功能

#### Model 层
- 定义领域模型数据结构
- 实现数据转换方法
- 定义业务实体间的关系和约束

#### Utils/Config 层
- 提供通用工具函数和配置
- 实现跨服务的共享功能
- 提供中间件和辅助组件

### 数据库操作规范

#### SQL 查询规范
- **强制要求：所有数据库查询只能使用 `sqlx::query!`、`sqlx::query_as!`、`sqlx::query_scalar!` 这三个宏版本，禁止使用非宏版本的函数（如 `sqlx::query`、`sqlx::query_as`、`sqlx::query_scalar` 等）**
- `sqlx::query_as!`：查询返回结构体时使用
- `sqlx::query_scalar!`：查询返回单个值时使用
- `sqlx::query!`：查询不返回结果时使用
- 所有参数必须通过参数绑定传递，禁止字符串拼接 SQL
- 使用枚举类型代替字符串常量

#### 参数绑定顺序
- 业务参数（ID、关键词等）
- 默认值/配置参数（如默认经纬度等）
- 分页参数（limit、cursor）

#### 数据库迁移规范
- 每个表一个文件
- 每次都用建表的方式而不是带有更改，因为会重建整个数据库

### 分页实现规范

#### 基本原则
1. 所有分页查询必须使用游标分页，禁止使用偏移量分页
2. 游标必须始终与表的 `id` 字段（自增主键，i64 类型）比较，禁止与 `user_id`、UUID 或其他业务主键比较
3. 返回的下一页游标值必须是查询结果中最后一条记录的 `id` 字段值
4. 所有消息、群组等相关分页必须用游标分页，且游标字段为表的自增主键 id

#### SQL 查询模式

- 降序（最新优先）：
  ```sql
  WHERE id < $cursor ORDER BY id DESC LIMIT $limit + 1
  ```
- 升序（最早优先）：
  ```sql
  WHERE id > $cursor ORDER BY id ASC LIMIT $limit + 1
  ```

#### 游标初始值处理
- 首次加载（无游标）时，默认游标值为 0，即：`cursor = 0 OR cursor IS NULL`
- SQL 中处理初始游标：`WHERE ($cursor = 0 OR id < $cursor)`
- 请求时通过传递上一次返回的游标获取下一页数据

#### 分页结果处理
1. 查询时总是请求 `limit + 1` 条记录
2. 如果返回记录数等于 `limit + 1`，则表示还有更多数据（`has_more = true`）
3. 返回给客户端的记录不包括额外查询的一条记录
4. 下一页游标应设置为最后一条返回记录的 `id` 字段值

#### 复杂排序情况处理
- 需要按非 `id` 字段排序时，必须同时按 `id` 进行次级排序以保证分页稳定性
- 正确写法示例（按时间戳降序）：
  ```sql
  WHERE (created_at = $timestamp AND id < $cursor) OR created_at < $timestamp
  ORDER BY created_at DESC, id DESC
  ```

#### 分页数据模型
```rust
pub struct PaginationParams {
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMeta,
}

pub struct PaginationMeta {
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}
```

#### 参考实现
- `user_service.rs`：`find_nearby_users`、`find_users_by_name`
- `group_service.rs`：`find_nearby_groups`、`find_groups_by_name`

### PostgreSQL 与 Rust 类型映射

#### 枚举类型映射
- 数据库中使用 enum 类型
- Rust 中使用对应的枚举定义并添加 sqlx 注解

#### 地理位置查询
- 使用 PostGIS 扩展进行地理位置查询
- 使用 `ST_Distance` 计算地理距离
- 使用 `::geography` 类型转换确保计算基于地理距离（以米为单位）
- 使用 `COALESCE` 处理可能的 NULL 值
- 使用空间索引优化查询性能

## 前端规范

### 依赖管理规则
- 不能直接修改 package.json 文件
- 使用 `pnpm add xxx` 增加依赖
- 使用 `pnpm add -D xxx` 增加开发依赖
- 使用 `pnpm remove xxx` 删除依赖
- 引入新依赖前，需联网查询该依赖包的最新信息，确认是否仍在维护、是否有安全问题
- 特别注意检查依赖包是否被弃用或有替代方案，避免使用过时技术
- 定期检查并更新项目依赖，保持与生态系统同步

### 目录结构规范
- 前端代码在 `frontend` 目录下
- 组件放在 `components` 目录，按功能分类
- 页面放在 `pages` 目录
- 布局放在 `layouts` 目录
- 状态管理在 `stores` 目录
- 公共工具函数在 `utils` 目录
- 类型定义在 `types` 目录
- 钩子函数在 `composables` 目录

### 组件命名规范
- 组件文件使用 PascalCase 命名（如：`UserProfile.vue`）
- 通用组件放在 `components/common` 目录下
- 业务组件按功能模块放在对应目录下
- 组件名应为多个单词（避免与 HTML 元素冲突）

### 图标使用规范
- 使用 Element Plus 官方图标库 `@element-plus/icons-vue`
- 通过通用 Icon 组件调用图标，不直接使用图标组件
- 新图标命名使用直接图标名称（如 `Plus`、`Search`），或 `ep:xxx` 格式
- 兼容旧 `ion:xxx-outline` 格式，内部映射到 Element Plus 图标
- 图标使用参考 `frontend/docs/icon-usage.md` 文档

### 布局与样式规范
- 使用 SCSS 作为 CSS 预处理器
- 使用 Element Plus 主题系统
- 组件样式使用 `scoped` 属性隔离
- 公共样式放在 `assets/scss` 目录下
- 使用 CSS 变量定义颜色和尺寸，避免硬编码
- 样式文件组织简洁，按功能分类

### 状态管理规范
- 使用 Pinia 进行状态管理
- Store 命名应为具体领域名词（如：`auth`、`user`）
- 每个 Store 单独一个文件，职责单一
- 异步操作用 actions 处理
- 使用 `pinia-plugin-persistedstate` 进行持久化

### 路由规范
- 使用 Nuxt Pages 自动路由
- 需中间件的页面用 `definePageMeta` 定义
- 路由命名用 kebab-case（如：`user-profile`）
- 布局通过 `layouts` 目录下文件定义

### 代码风格规范
- 使用 TypeScript 编写代码
- 使用 Composition API 和 `<script setup>` 语法
- 遵循 ESLint 规则
- 所有属性、方法、变量用驼峰命名法
- 方法名用动词开头（如：`fetchData`、`updateUser`）
- 保持代码简洁，避免过度嵌套和复杂逻辑

### API 调用规范
- 使用 Fetch API 发起请求
- API 调用统一在 `services` 目录下封装
- 接口基础 URL 通过环境变量配置
- 接口返回数据类型要定义清楚
- 错误处理要完善

### 性能优化规范
- 组件合理拆分，避免过大组件
- 列表使用虚拟滚动优化
- 大数据操作用 Web Worker
- 图片使用懒加载
- 按需导入第三方库
- 删除不必要依赖，减少打包体积
- 使用轻量级工具函数，避免引入大型库

### Nuxt 配置规范
- 使用 `nuxt.config.ts` 进行全局配置
- 插件放在 `plugins` 目录下
- 中间件放在 `middleware` 目录下
- 静态资源放在 `public` 目录下
- 合理配置构建选项，优化打包体积
- 使用按需导入减少不必要代码

### Nuxt 文件处理规范
- **不要直接替换** Nuxt 自动生成的文件
- 在 Nuxt 生成文件基础上修改和扩展，保留原始结构和元数据
- 修改生成文件时，确保不影响 Nuxt 核心功能
- 使用 `<script setup>` 和 `<template>` 添加自定义功能
- 如需添加中间件或页面元数据，使用 `definePageMeta` 而非替换文件

### Element Plus 配置规范
- 在插件中配置全局主题和国际化
- 组件按需导入，禁止全量引入
- 使用 transpile 配置确保 SSR 正常工作
- 深色模式用 CSS 类和变量方式实现
- 主题变量与 CSS 变量保持一致

### Nuxt `.client.ts` / `.server.ts` 文件命名与使用规范

#### 适用目录
- 仅允许在 Nuxt 约定的以下目录下使用 `.client.ts`、`.server.ts`、`.client.vue`、`.server.vue` 等后缀：
  - `pages/`
  - `layouts/`
  - `middleware/`
  - `plugins/`
  - `composables/`
  - `components/`
  - `app.vue`（顶层文件可用）

#### 规范说明
- 上述目录下，`.client.ts`/`.client.vue` 只在客户端加载和执行，`.server.ts`/`.server.vue` 只在服务端加载和执行
- 在 `utils/`、`services/`、`stores/` 等自定义目录下，**禁止**使用 `.client.ts`、`.server.ts` 等后缀，否则 Nuxt 不识别，可能导致运行时错误或 SSR 问题
- 需区分环境的逻辑，必须放在 Nuxt 约定目录下，并用 `.client` 或 `.server` 后缀
- 普通工具、服务、数据文件请直接用常规命名（如 `api.ts`、`xxx.service.ts`）

#### 使用示例
- 正确：`components/Comments.client.vue`、`plugins/analytics.client.ts`、`composables/useWindowSize.client.ts`、`utils/api.ts`
- 错误：`utils/api.client.ts`、`services/auth.server.ts`

#### 配对使用案例
- 同时存在 `.client` 和 `.server` 版本时，Nuxt 会：
  - 服务端渲染用 `.server` 版本
  - 客户端渲染/激活用 `.client` 版本

```
components/
  ├─ Comments.client.vue  // 客户端版本
  └─ Comments.server.vue  // 服务端版本
```