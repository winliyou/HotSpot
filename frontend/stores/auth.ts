import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authService } from '~/utils/services/auth-service'
import type { LoginRequest, RegisterRequest, CreateTempUserRequest, AuthResponse } from '~/types/user'

export const useAuthStore = defineStore('auth', () => {
  // 状态
  const user = ref<{ user_id: number; nickname: string } | null>(null)
  const token = ref<string | null>(null)
  const refreshToken = ref<string | null>(null)
  const expiresAt = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  
  // 计算属性
  const isAuthenticated = computed(() => !!token.value && !!user.value)
  const getToken = computed(() => token.value)
  const getUser = computed(() => user.value)
  
  // 内部方法：设置认证数据
  function setAuthData(authData: AuthResponse) {
    user.value = {
      user_id: authData.user_id,
      nickname: authData.nickname
    }
    token.value = authData.access_token
    refreshToken.value = authData.refresh_token
    expiresAt.value = authData.expires_at
    error.value = null
    localStorage.setItem('access_token', authData.access_token)
    localStorage.setItem('refresh_token', authData.refresh_token)
  }
  
  // 内部方法：清除认证数据
  function clearAuthData() {
    user.value = null
    token.value = null
    refreshToken.value = null
    expiresAt.value = null
    localStorage.removeItem('access_token')
    localStorage.removeItem('refresh_token')
  }
  
  // Action: 登录
  async function login(credentials: LoginRequest) {
    isLoading.value = true
    error.value = null
    try {
      const response = await authService.login(credentials)
      setAuthData(response)
      return response
    } catch (err: any) {
      error.value = err.message || '登录失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 注册
  async function register(userData: RegisterRequest) {
    isLoading.value = true
    error.value = null
    try {
      const response = await authService.register(userData)
      setAuthData(response)
      return response
    } catch (err: any) {
      error.value = err.message || '注册失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 创建临时用户
  async function createTempUser(userData?: CreateTempUserRequest) {
    isLoading.value = true
    error.value = null
    try {
      const response = await authService.createTempUser(userData)
      setAuthData(response)
      return response
    } catch (err: any) {
      error.value = err.message || '创建临时用户失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 刷新令牌
  async function refreshTokenAction() {
    if (!refreshToken.value) {
      throw new Error('刷新令牌不存在')
    }
    isLoading.value = true
    try {
      const response = await authService.refreshToken(refreshToken.value)
      setAuthData(response)
      return response
    } catch (err: any) {
      error.value = err.message || '刷新令牌失败'
      clearAuthData()
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 登出
  function logout() {
    clearAuthData()
  }
  
  // Action: 判断 token 是否过期
  function isTokenExpired(): boolean {
    if (!expiresAt.value) return true
    const expiryTime = new Date(expiresAt.value).getTime()
    const currentTime = new Date().getTime()
    return expiryTime - currentTime < 5 * 60 * 1000
  }
  
  // Action: 检查并自动刷新 token
  async function checkAndRefreshToken(): Promise<string | null> {
    if (!token.value) return null
    if (isTokenExpired() && refreshToken.value) {
      try {
        await refreshTokenAction()
      } catch (err) {
        return null
      }
    }
    return token.value
  }
  
  return {
    user,
    token,
    refreshToken,
    expiresAt,
    isLoading,
    error,
    isAuthenticated,
    getToken,
    getUser,
    login,
    register,
    createTempUser,
    refreshTokenAction,
    logout,
    isTokenExpired,
    checkAndRefreshToken
  }
}, {
  persist: true
}) 