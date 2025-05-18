import { useAuthStore } from '~/stores/auth'
import type { LoginRequest, RegisterRequest, CreateTempUserRequest } from '~/types/user'

export function useAuthClient() {
  const authStore = useAuthStore()

  async function login(credentials: LoginRequest) {
    await authStore.login(credentials)
  }

  async function register(userData: RegisterRequest) {
    await authStore.register(userData)
  }

  async function createTempUser(userData?: CreateTempUserRequest) {
    await authStore.createTempUser(userData)
  }

  async function refreshTokenAction() {
    await authStore.refreshTokenAction()
  }

  function logout() {
    authStore.logout()
  }

  function isTokenExpired(): boolean {
    return authStore.isTokenExpired()
  }

  async function checkAndRefreshToken(): Promise<string | null> {
    return await authStore.checkAndRefreshToken()
  }

  return {
    login,
    register,
    createTempUser,
    refreshTokenAction,
    logout,
    isTokenExpired,
    checkAndRefreshToken
  }
} 