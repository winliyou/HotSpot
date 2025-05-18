import { apiPost } from '../api'
import type {
  GroupInfo,
  CreateGroupRequest,
  CreateGroupResponse,
  JoinGroupRequest,
  JoinGroupResponse,
  LeaveGroupRequest,
  LeaveGroupResponse,
  GroupMembersRequest,
  GroupMember
} from '~/types/group'
import type { PaginatedResponse } from '../api'

/**
 * 群组服务，提供群组相关功能
 */
export const groupService = {
  /**
   * 创建群组
   * @param data 群组创建参数
   * @returns 群组ID
   */
  async createGroup(data: CreateGroupRequest): Promise<CreateGroupResponse> {
    return await apiPost<CreateGroupResponse>('/group/create', data)
  },

  /**
   * 加入群组
   * @param data 加入群组参数
   * @returns 加入结果
   */
  async joinGroup(data: JoinGroupRequest): Promise<JoinGroupResponse> {
    return await apiPost<JoinGroupResponse>('/group/join', data)
  },

  /**
   * 退出群组
   * @param data 退出群组参数
   * @returns 退出结果
   */
  async leaveGroup(data: LeaveGroupRequest): Promise<LeaveGroupResponse> {
    return await apiPost<LeaveGroupResponse>('/group/leave', data)
  },

  /**
   * 获取群组信息
   * @param groupId 群组ID
   * @returns 群组信息
   */
  async getGroupInfo(groupId: string): Promise<GroupInfo> {
    return await apiPost<GroupInfo>('/group/info', { group_id: groupId })
  },

  /**
   * 获取群组成员
   * @param data 群组成员请求参数
   * @returns 群组成员列表
   */
  async getGroupMembers(data: GroupMembersRequest): Promise<PaginatedResponse<GroupMember>> {
    return await apiPost<PaginatedResponse<GroupMember>>('/group/members', data)
  },

  /**
   * 获取用户加入的所有群组
   * @param userId 可选的用户ID，不传则获取当前用户的群组
   * @returns 群组列表
   */
  async getUserGroups(userId?: number): Promise<PaginatedResponse<GroupInfo>> {
    return await apiPost<PaginatedResponse<GroupInfo>>('/group/user_groups', {
      user_id: userId,
      pagination: {
        limit: 10
      }
    })
  },

  /**
   * 获取附近的群组
   * @param latitude 纬度
   * @param longitude 经度
   * @param radius 半径(米)
   * @param cursor 分页游标
   * @param limit 每页数量
   * @returns 群组列表
   */
  async getNearbyGroups(
    latitude: number, 
    longitude: number, 
    radius: number = 5000,
    cursor?: string | number,
    limit: number = 10
  ): Promise<PaginatedResponse<GroupInfo>> {
    return await apiPost<PaginatedResponse<GroupInfo>>('/group/nearby', {
      latitude,
      longitude,
      radius,
      pagination: {
        cursor,
        limit
      }
    })
  },

  /**
   * 获取最近加入的群组
   * @returns 最近加入的群组
   */
  async getRecentJoinedGroups(): Promise<GroupInfo[]> {
    return await apiPost<GroupInfo[]>('/group/recent_joined')
  },

  /**
   * 按名称搜索群组
   * @param name 群组名称关键词
   * @param cursor 分页游标
   * @param limit 每页数量
   * @returns 群组列表
   */
  async findGroupsByName(name: string, cursor?: string | number, limit: number = 10): Promise<PaginatedResponse<GroupInfo>> {
    return await apiPost<PaginatedResponse<GroupInfo>>('/group/search_by_name', {
      name,
      pagination: {
        cursor,
        limit
      }
    })
  }
}