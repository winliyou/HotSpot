import { useUserStore } from '~/stores/user'
import { userService } from '~/utils/services/user-service'
import type { User, UpdateLocationRequest } from '~/types/user'
import type { PaginatedResponse } from '~/utils/api'

export function useUser() {
  const userStore = useUserStore()
  const { nearbyUsers, currentLocation, isLoadingNearby, hasMoreNearby, nextCursor, searchResults, currentUser, defaultRadius, error } = userStore

  // 设置当前位置
  function setCurrentLocation(latitude: number, longitude: number, locationName: string) {
    userStore.setCurrentLocation(latitude, longitude, locationName)
  }

  // 清除当前位置
  function clearCurrentLocation() {
    userStore.clearCurrentLocation()
  }

  // 更新用户位置到服务器
  async function updateLocationToServer() {
    await userStore.updateLocationToServer()
  }

  // 获取附近用户
  async function fetchNearbyUsers(refresh = false) {
    await userStore.fetchNearbyUsers(refresh)
  }

  // 搜索用户
  async function searchUsersByName(keyword: string) {
    await userStore.searchUsersByName(keyword)
  }

  // 获取用户详情
  async function getUserById(userId: number) {
    await userStore.getUserById(userId)
  }

  // 获取设备位置
  async function getDeviceLocation(): Promise<{latitude: number, longitude: number} | null> {
    return await userStore.getDeviceLocation()
  }

  return {
    setCurrentLocation,
    clearCurrentLocation,
    updateLocationToServer,
    fetchNearbyUsers,
    searchUsersByName,
    getUserById,
    getDeviceLocation
  }
} 