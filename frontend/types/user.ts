// 用户基本信息
export interface User {
  user_id: number
  nickname: string
  last_active?: string
  latitude?: number
  longitude?: number
  distance?: number
  location_name?: string
  online_status?: string
}

// 认证响应
export interface AuthResponse {
  user_id: number
  nickname: string
  access_token: string
  refresh_token: string
  expires_at: string
}

// 登录请求
export interface LoginRequest {
  login_id: string
  password: string
}

// 临时用户创建请求
export interface CreateTempUserRequest {
  nickname?: string
}

// 用户详情请求
export interface SearchUserByIdRequest {
  user_id: number
}

// 附近用户查询请求
export interface NearbyUsersRequest {
  latitude: number
  longitude: number
  radius?: number
  pagination?: {
    limit?: number
    cursor?: number
  }
}

// 用户位置更新请求
export interface UpdateLocationRequest {
  latitude: number
  longitude: number
  location_name: string
}

// 用户名称搜索请求
export interface SearchUserByNameRequest {
  keyword: string
  pagination?: {
    limit?: number
    cursor?: number
  }
}

// 用户注册请求
export interface RegisterRequest {
  login_id: string
  password: string
  nickname: string
  email: string
  confirm_password?: string
} 