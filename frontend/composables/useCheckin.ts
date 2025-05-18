import { useCheckinStore } from '~/stores/checkin'
import { checkinService } from '~/utils/services/checkin-service'
import type { CheckInInfo, CreateCheckinRequest } from '~/types/checkin'
import type { PaginatedResponse } from '~/utils/api'

export function useCheckin() {
  const store = useCheckinStore()

  // 创建签到
  async function createCheckin(data: CreateCheckinRequest) {
    await store.createCheckin(data)
  }

  // 删除签到
  async function deleteCheckin(checkinId: string) {
    await store.deleteCheckin(checkinId)
  }

  // 获取签到详情
  async function getCheckinDetail(checkinId: string) {
    await store.getCheckinDetail(checkinId)
  }

  // 获取附近签到
  async function fetchNearbyCheckins(latitude: number, longitude: number, radius: number, refresh = false) {
    await store.fetchNearbyCheckins(latitude, longitude, radius, refresh)
  }

  // 获取用户签到历史
  async function getUserCheckinHistory(userId?: number, refresh = false) {
    await store.getUserCheckinHistory(userId, refresh)
  }

  // 按标签搜索签到
  async function searchCheckinsByTags(tags: string[], latitude: number, longitude: number) {
    await store.searchCheckinsByTags(tags, latitude, longitude)
  }

  // 点赞签到
  async function likeCheckin(checkinId: string) {
    await store.likeCheckin(checkinId)
  }

  // 取消点赞
  async function unlikeCheckin(checkinId: string) {
    await store.unlikeCheckin(checkinId)
  }

  return {
    createCheckin,
    deleteCheckin,
    getCheckinDetail,
    fetchNearbyCheckins,
    getUserCheckinHistory,
    searchCheckinsByTags,
    likeCheckin,
    unlikeCheckin
  }
} 