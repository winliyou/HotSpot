import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { userService } from '~/utils/services/user-service'
import { DEFAULT_LATITUDE, DEFAULT_LONGITUDE } from '~/utils/constants'
import type { User, UpdateLocationRequest } from '~/types/user'
import type { PaginatedResponse } from '~/types/pagination'

// 定义默认位置常量（南天门）
const DEFAULT_LOCATION_NAME = '南天门'

export const useUserStore = defineStore('user', () => {
  // 状态
  const nearbyUsers = ref<User[]>([])
  const currentLocation = ref<{ latitude: number; longitude: number; locationName: string } | null>(null)
  const isLoadingNearby = ref(false)
  const hasMoreNearby = ref(false)
  const nextCursor = ref<number | null>(null)
  const searchResults = ref<User[]>([])
  const currentUser = ref<User | null>(null)
  const defaultRadius = ref(1000)
  const error = ref<string | null>(null)

  // 计算属性
  const getNearbyUsers = computed(() => nearbyUsers.value)
  const getCurrentLocation = computed(() => currentLocation.value)
  const getSearchResults = computed(() => searchResults.value)
  const hasLocation = computed(() => !!currentLocation.value)

  // Action: 设置当前位置
  function setCurrentLocation(latitude: number, longitude: number, locationName: string) {
    currentLocation.value = { latitude, longitude, locationName }
  }

  // Action: 清除当前位置
  function clearCurrentLocation() {
    currentLocation.value = null
  }

  // Action: 更新用户位置到服务器
  async function updateLocationToServer() {
    if (!currentLocation.value) return
    try {
      const updateData: UpdateLocationRequest = {
        latitude: currentLocation.value.latitude,
        longitude: currentLocation.value.longitude,
        location_name: currentLocation.value.locationName
      }
      await userService.updateLocation(updateData)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '更新位置失败'
      throw err
    }
  }

  // Action: 获取附近用户
  async function fetchNearbyUsers(refresh = false) {
    if (!currentLocation.value) {
      error.value = '未设置当前位置'
      return
    }
    if (refresh) {
      nearbyUsers.value = []
      nextCursor.value = null
    }
    isLoadingNearby.value = true
    try {
      const response = await userService.searchByLocation({
        latitude: currentLocation.value.latitude,
        longitude: currentLocation.value.longitude,
        radius: defaultRadius.value,
        pagination: {
          cursor: nextCursor.value || undefined,
          limit: 10
        }
      })
      handleUsersResponse(response, refresh)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取附近用户失败'
      throw err
    } finally {
      isLoadingNearby.value = false
    }
  }

  // Action: 搜索用户
  async function searchUsersByName(keyword: string) {
    try {
      const response = await userService.searchByName({
        keyword,
        pagination: {
          limit: 10
        }
      })
      searchResults.value = response.items
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '搜索用户失败'
      throw err
    }
  }

  // Action: 获取用户详情
  async function getUserById(userId: number) {
    try {
      const user = await userService.searchById({ user_id: userId })
      currentUser.value = user
      error.value = null
      return user
    } catch (err: any) {
      error.value = err.message || '获取用户详情失败'
      throw err
    }
  }

  // Action: 处理用户列表响应
  function handleUsersResponse(response: PaginatedResponse<User>, refresh: boolean) {
    if (refresh) {
      nearbyUsers.value = response.items
    } else {
      nearbyUsers.value = [...nearbyUsers.value, ...response.items]
    }
    hasMoreNearby.value = response.pagination.has_more
    nextCursor.value = typeof response.pagination.next_cursor === 'number' 
      ? response.pagination.next_cursor 
      : response.pagination.next_cursor 
        ? parseInt(response.pagination.next_cursor.toString(), 10)
        : null
  }

  // Action: 获取设备位置
  async function getDeviceLocation(): Promise<{latitude: number, longitude: number} | null> {
    return new Promise((resolve, reject) => {
      if (!navigator.geolocation) {
        error.value = '您的浏览器不支持地理位置服务'
        reject(new Error('浏览器不支持地理位置服务'))
        return
      }
      navigator.geolocation.getCurrentPosition(
        (position) => {
          const coords = {
            latitude: position.coords.latitude,
            longitude: position.coords.longitude
          }
          resolve(coords)
        },
        (positionError) => {
          let message = '获取位置失败'
          switch (positionError.code) {
            case GeolocationPositionError.PERMISSION_DENIED:
              message = '用户拒绝了位置请求'
              break
            case GeolocationPositionError.POSITION_UNAVAILABLE:
              message = '位置信息不可用'
              break
            case GeolocationPositionError.TIMEOUT:
              message = '获取位置超时'
              break
          }
          error.value = message
          // 获取位置失败时，使用默认位置（南天门）
          resolve({
            latitude: DEFAULT_LATITUDE,
            longitude: DEFAULT_LONGITUDE
          })
        },
        { timeout: 10000, enableHighAccuracy: true }
      )
    })
  }

  return {
    nearbyUsers,
    currentLocation,
    isLoadingNearby,
    hasMoreNearby,
    nextCursor,
    searchResults,
    currentUser,
    defaultRadius,
    error,
    getNearbyUsers,
    getCurrentLocation,
    getSearchResults,
    hasLocation,
    setCurrentLocation,
    clearCurrentLocation,
    updateLocationToServer,
    fetchNearbyUsers,
    searchUsersByName,
    getUserById,
    getDeviceLocation
  }
}, {
  persist: true
})