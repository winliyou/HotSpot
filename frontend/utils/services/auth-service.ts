import { apiPost } from '../api'
import type { 
  AuthResponse,
  CreateTempUserRequest,
  LoginRequest,
  RegisterRequest
} from '~/types/user'

/**
 * 认证服务，提供用户认证相关功能
 */
export const authService = {
  /**
   * 用户登录
   * @param data 登录参数
   * @returns 认证响应
   */
  async login(data: LoginRequest): Promise<AuthResponse> {
    return await apiPost<AuthResponse>('/user/login', data)
  },

  /**
   * 刷新令牌
   * @param refreshToken 刷新令牌
   * @returns 认证响应
   */
  async refreshToken(refreshToken: string): Promise<AuthResponse> {
    return await apiPost<AuthResponse>('/user/refresh-token', { refresh_token: refreshToken })
  },

  /**
   * 用户注册
   * @param data 注册参数
   * @returns 认证响应
   */
  async register(data: RegisterRequest): Promise<AuthResponse> {
    return await apiPost<AuthResponse>('/user/register', data)
  },

  /**
   * 创建临时用户
   * @param data 临时用户参数
   * @returns 认证响应
   */
  async createTempUser(data?: CreateTempUserRequest): Promise<AuthResponse> {
    return await apiPost<AuthResponse>('/user/create_temp_user', data || {})
  }
} 