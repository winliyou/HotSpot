import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { CheckInInfo, CreateCheckinRequest } from '~/types/checkin'
import { checkinService } from '~/utils/services/checkin-service'

export const useCheckinStore = defineStore('checkin', () => {
  // 状态
  const checkins = ref<CheckInInfo[]>([])
  const userCheckins = ref<CheckInInfo[]>([])
  const currentCheckin = ref<CheckInInfo | null>(null)
  const isLoading = ref(false)
  const hasMore = ref(false)
  const nextCursor = ref<number | null>(null)
  const error = ref<string | null>(null)

  // 计算属性
  const checkinsList = computed(() => checkins.value)
  const userCheckinsList = computed(() => userCheckins.value)
  const currentCheckinData = computed(() => currentCheckin.value)

  // Action: 创建签到
  async function createCheckin(data: CreateCheckinRequest) {
    isLoading.value = true
    try {
      const response = await checkinService.create(data)
      // 这里假设 response 返回的是 { checkin_id: string }，需要再查详情
      const detail = await checkinService.searchById({ checkin_id: response.checkin_id })
      checkins.value = [detail, ...checkins.value]
      error.value = null
      return detail
    } catch (err: any) {
      error.value = err.message || '创建签到失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 删除签到
  async function deleteCheckin(checkinId: string) {
    isLoading.value = true
    try {
      await checkinService.delete({ checkin_id: checkinId })
      checkins.value = checkins.value.filter(c => c.checkin_id !== checkinId)
      userCheckins.value = userCheckins.value.filter(c => c.checkin_id !== checkinId)
      if (currentCheckin.value?.checkin_id === checkinId) {
        currentCheckin.value = null
      }
      error.value = null
    } catch (err: any) {
      error.value = err.message || '删除签到失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 获取签到详情
  async function getCheckinDetail(checkinId: string) {
    isLoading.value = true
    try {
      const checkin = await checkinService.searchById({ checkin_id: checkinId })
      currentCheckin.value = checkin
      error.value = null
      return checkin
    } catch (err: any) {
      error.value = err.message || '获取签到详情失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 获取附近签到
  async function fetchNearbyCheckins(latitude: number, longitude: number, radius: number, refresh = false) {
    isLoading.value = true
    try {
      const response = await checkinService.searchByLocation({ latitude, longitude, radius })
      checkins.value = response.items
      hasMore.value = response.pagination.has_more
      nextCursor.value = response.pagination.next_cursor || null
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '获取附近签到失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 获取用户签到历史
  async function getUserCheckinHistory(userId?: number, refresh = false) {
    isLoading.value = true
    try {
      const response = await checkinService.getUserHistory({
        user_id: userId,
        cursor: refresh ? undefined : nextCursor.value || undefined,
        limit: 10
      })
      userCheckins.value = response.items
      hasMore.value = response.pagination.has_more
      nextCursor.value = response.pagination.next_cursor || null
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '获取签到历史失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 按标签搜索签到
  async function searchCheckinsByTags(tags: string[], latitude: number, longitude: number) {
    isLoading.value = true
    try {
      const response = await checkinService.searchByTags({
        tags,
        latitude,
        longitude,
        cursor: undefined,
        limit: 10
      })
      checkins.value = response.items
      hasMore.value = response.pagination.has_more
      nextCursor.value = response.pagination.next_cursor || null
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '搜索签到失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 点赞签到
  async function likeCheckin(checkinId: string) {
    try {
      await checkinService.like(checkinId)
      updateCheckinLikeStatus(checkinId, true)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '点赞失败'
      throw err
    }
  }

  // Action: 取消点赞
  async function unlikeCheckin(checkinId: string) {
    try {
      await checkinService.unlike(checkinId)
      updateCheckinLikeStatus(checkinId, false)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '取消点赞失败'
      throw err
    }
  }

  // 内部方法：更新点赞状态
  function updateCheckinLikeStatus(checkinId: string, liked: boolean) {
    if (currentCheckin.value && currentCheckin.value.checkin_id === checkinId) {
      currentCheckin.value.liked_by_me = liked
      currentCheckin.value.likes_count += liked ? 1 : -1
    }
    checkins.value = checkins.value.map(c => {
      if (c.checkin_id === checkinId) {
        return {
          ...c,
          liked_by_me: liked,
          likes_count: c.likes_count + (liked ? 1 : -1)
        }
      }
      return c
    })
    userCheckins.value = userCheckins.value.map(c => {
      if (c.checkin_id === checkinId) {
        return {
          ...c,
          liked_by_me: liked,
          likes_count: c.likes_count + (liked ? 1 : -1)
        }
      }
      return c
    })
  }

  return {
    checkins,
    userCheckins,
    currentCheckin,
    isLoading,
    hasMore,
    nextCursor,
    error,
    getNearbyCheckins: checkinsList,
    getUserCheckins: userCheckinsList,
    getCurrentCheckin: currentCheckinData,
    createCheckin,
    deleteCheckin,
    getCheckinDetail,
    fetchNearbyCheckins,
    getUserCheckinHistory,
    searchCheckinsByTags,
    likeCheckin,
    unlikeCheckin
  }
}, {
  persist: true
}) 