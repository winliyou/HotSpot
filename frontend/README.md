# HotSpot 前端项目

HotSpot 是一个地理位置社交应用，允许用户分享位置、创建签到、加入群组和与附近的人交流。

## 技术栈

- Vue 3 - 渐进式JavaScript框架
- Nuxt 3 - Vue.js的服务端渲染框架
- Element Plus - Vue 3组件库
- Pinia - Vue状态管理库
- SCSS - CSS预处理器
- TypeScript - JavaScript的超集，提供类型系统

## 项目结构

```
frontend/
├── assets/              # 静态资源文件
│   └── styles/          # 全局样式文件
├── components/          # 组件目录
│   ├── auth/            # 认证相关组件
│   ├── checkin/         # 签到相关组件
│   ├── common/          # 通用组件
│   ├── group/           # 群组相关组件
│   └── user/            # 用户相关组件
├── composables/         # 可复用的组合式函数
├── layouts/             # 布局组件
├── pages/               # 页面组件（自动生成路由）
├── public/              # 公共静态资源
├── stores/              # Pinia状态管理
├── types/               # TypeScript类型定义
└── utils/               # 工具函数
    ├── api.ts           # API请求工具
    └── services/        # API服务模块
```

## 功能模块

- **认证系统**：用户登录、注册、创建临时用户
- **签到系统**：创建签到、查看附近签到、查看用户签到历史
- **群组系统**：创建群组、加入群组、群组聊天
- **消息系统**：群组消息、私聊消息
- **位置系统**：获取用户位置、显示附近的人和签到

## 开发指南

### 安装依赖

```bash
# 使用pnpm安装依赖
pnpm install
```

### 开发环境

```bash
# 启动开发服务器
pnpm dev
```

### 编译构建

```bash
# 构建生产环境版本
pnpm build
```

### 代码风格和规范

- 使用Vue 3的组合式API和`<script setup>`语法
- 使用TypeScript进行类型检查
- 使用SCSS编写样式，优先使用CSS变量和响应式工具类
- 遵循Element Plus的设计规范

## 环境变量

创建`.env`文件配置以下环境变量:

```env
# API基础URL
API_BASE_URL=http://localhost:3000/api/v1
# WebSocket URL
WS_BASE_URL=ws://localhost:3000/ws
```

## 状态管理

项目使用Pinia进行状态管理，主要store包括:

- `auth.ts` - 用户认证状态
- `user.ts` - 用户信息和位置
- `checkin.ts` - 签到数据
- `group.ts` - 群组数据
- `message.ts` - 消息数据

## 响应式设计

- 使用CSS变量定义主题和断点
- 使用媒体查询适配不同设备
- 桌面优先设计，针对移动设备进行优化

## 贡献指南

1. 遵循Git分支开发流程
2. 提交前进行代码格式化和类型检查
3. 测试新功能，确保跨浏览器兼容性
4. 依赖管理使用`pnpm add`和`pnpm remove`命令，不直接修改package.json
