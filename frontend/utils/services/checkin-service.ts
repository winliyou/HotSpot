import { apiPost } from '../api'
import type {
  CheckInInfo,
  CreateCheckinRequest,
  CreateCheckinResponse,
  DeleteCheckinRequest,
  DeleteCheckinResponse,
  SearchCheckInByIdRequest,
  SearchCheckinsByLocationRequest,
  SearchCheckinsByTagsRequest,
  UserCheckinsHistoryRequest
} from '~/types/checkin'
import type { PaginatedResponse } from '../api'

/**
 * 签到服务，提供签到相关功能
 */
export const checkinService = {
  /**
   * 创建签到
   * @param data 签到参数
   * @returns 签到ID
   */
  async create(data: CreateCheckinRequest): Promise<CreateCheckinResponse> {
    return await apiPost<CreateCheckinResponse>('/checkin/create', data)
  },

  /**
   * 删除签到
   * @param data 签到ID参数
   * @returns 删除结果
   */
  async delete(data: DeleteCheckinRequest): Promise<DeleteCheckinResponse> {
    return await apiPost<DeleteCheckinResponse>('/checkin/delete', data)
  },

  /**
   * 按ID查询签到
   * @param data 签到ID参数
   * @returns 签到详情
   */
  async searchById(data: SearchCheckInByIdRequest): Promise<CheckInInfo> {
    return await apiPost<CheckInInfo>('/checkin/search_by_id', data)
  },

  /**
   * 按位置搜索签到
   * @param data 位置参数
   * @returns 分页签到列表
   */
  async searchByLocation(data: SearchCheckinsByLocationRequest): Promise<PaginatedResponse<CheckInInfo>> {
    return await apiPost<PaginatedResponse<CheckInInfo>>('/checkin/search_by_location', {
      latitude: data.latitude,
      longitude: data.longitude,
      radius: data.radius,
      cursor: data.cursor,
      limit: data.limit || 10
    })
  },

  /**
   * 查询用户签到历史
   * @param data 用户ID和分页参数
   * @returns 分页签到列表
   */
  async getUserHistory(data: UserCheckinsHistoryRequest): Promise<PaginatedResponse<CheckInInfo>> {
    return await apiPost<PaginatedResponse<CheckInInfo>>('/checkin/history', {
      user_id: data.user_id,
      pagination: {
        cursor: data.cursor,
        limit: data.limit || 10
      }
    })
  },

  /**
   * 按标签搜索签到
   * @param data 标签参数
   * @returns 分页签到列表
   */
  async searchByTags(data: SearchCheckinsByTagsRequest): Promise<PaginatedResponse<CheckInInfo>> {
    return await apiPost<PaginatedResponse<CheckInInfo>>('/checkin/search_by_tags', {
      tags: data.tags,
      latitude: data.latitude,
      longitude: data.longitude,
      cursor: data.cursor,
      limit: data.limit || 10
    })
  },

  /**
   * 点赞签到
   * @param checkinId 签到ID
   * @returns void
   */
  async like(checkinId: string): Promise<void> {
    return await apiPost<void>('/checkin/like', { checkin_id: checkinId })
  },

  /**
   * 取消点赞
   * @param checkinId 签到ID
   * @returns void
   */
  async unlike(checkinId: string): Promise<void> {
    return await apiPost<void>('/checkin/unlike', { checkin_id: checkinId })
  }
} 