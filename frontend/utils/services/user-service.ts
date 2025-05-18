import { apiPost } from '../api'
import type {
  User,
  NearbyUsersRequest,
  SearchUserByIdRequest,
  SearchUserByNameRequest,
  UpdateLocationRequest
} from '~/types/user'
import type { PaginatedResponse } from '../api'

/**
 * 用户服务，提供用户相关功能
 */
export const userService = {
  /**
   * 按ID查询用户
   * @param data 用户ID参数
   * @returns 用户信息
   */
  async searchById(data: SearchUserByIdRequest): Promise<User> {
    return await apiPost<User>('/user/search_by_id', data)
  },

  /**
   * 查询附近用户
   * @param data 位置参数
   * @returns 分页用户列表
   */
  async searchByLocation(data: NearbyUsersRequest): Promise<PaginatedResponse<User>> {
    return await apiPost<PaginatedResponse<User>>('/user/search_by_location', {
      latitude: data.latitude,
      longitude: data.longitude,
      radius: data.radius || 1000,
      pagination: {
        limit: data.pagination?.limit || 10,
        cursor: data.pagination?.cursor
      }
    })
  },

  /**
   * 按名称搜索用户
   * @param data 搜索参数
   * @returns 分页用户列表
   */
  async searchByName(data: SearchUserByNameRequest): Promise<PaginatedResponse<User>> {
    return await apiPost<PaginatedResponse<User>>('/user/search_by_name', {
      keyword: data.keyword,
      pagination: {
        limit: data.pagination?.limit || 10,
        cursor: data.pagination?.cursor
      }
    })
  },

  /**
   * 更新用户位置
   * @param data 位置参数
   * @returns 空响应
   */
  async updateLocation(data: UpdateLocationRequest): Promise<void> {
    return await apiPost<void>('/user/update_location', data)
  }
} 