import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { groupService } from '~/utils/services/group-service'
import type { GroupInfo, GroupMember } from '~/types/group'
import type { PaginatedResponse } from '~/utils/api'
import type { Ref } from 'vue'

export const useGroupStore = defineStore('group', () => {
  // 状态
  const userGroups = ref<GroupInfo[]>([])
  const nearbyGroups = ref<GroupInfo[]>([])
  const currentGroup = ref<GroupInfo | null>(null)
  const groupMembers = ref<GroupMember[]>([])
  const recentJoinedGroups = ref<GroupInfo[]>([])
  const isLoading = ref(false)
  const isLoadingMembers = ref(false)
  const isCreating = ref(false)
  const hasMoreGroups = ref(false)
  const hasMoreMembers = ref(false)
  const nextGroupsCursor = ref<string | number | null>(null)
  const nextMembersCursor = ref<number | null>(null)
  const error = ref<string | null>(null)

  // 计算属性
  const getUserGroups = computed(() => userGroups.value)
  const getNearbyGroups = computed(() => nearbyGroups.value)
  const getCurrentGroup = computed(() => currentGroup.value)
  const getGroupMembers = computed(() => groupMembers.value)
  const getRecentJoinedGroups = computed(() => recentJoinedGroups.value)
  
  // Action: 创建群组
  async function createGroup(name: string, description: string, location: { latitude: number; longitude: number; locationName: string }) {
    isCreating.value = true
    try {
      const response = await groupService.createGroup({
        name,
        description,
        latitude: location.latitude,
        longitude: location.longitude,
        location_name: location.locationName
      })
      
      error.value = null
      return response.group_id
    } catch (err: any) {
      error.value = err.message || '创建群组失败'
      throw err
    } finally {
      isCreating.value = false
    }
  }
  
  // Action: 获取用户的群组
  async function fetchUserGroups(refresh = false) {
    if (refresh) {
      userGroups.value = []
      nextGroupsCursor.value = null
    }
    
    isLoading.value = true
    try {
      const response = await groupService.getUserGroups()
      
      // 处理响应数据
      handleGroupsResponse(response, userGroups, refresh)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取群组失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 获取附近的群组
  async function fetchNearbyGroups(
    latitude: number,
    longitude: number,
    radius: number = 5000,
    refresh = false
  ) {
    if (refresh) {
      nearbyGroups.value = []
      nextGroupsCursor.value = null
    }
    
    isLoading.value = true
    try {
      const response = await groupService.getNearbyGroups(
        latitude,
        longitude,
        radius,
        nextGroupsCursor.value || undefined
      )
      
      // 处理响应数据
      handleGroupsResponse(response, nearbyGroups, refresh)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取附近群组失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 获取群组详情
  async function getGroupDetail(groupId: string) {
    isLoading.value = true
    try {
      const groupInfo = await groupService.getGroupInfo(groupId)
      currentGroup.value = groupInfo
      error.value = null
      return groupInfo
    } catch (err: any) {
      error.value = err.message || '获取群组详情失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }
  
  // Action: 获取群组成员
  async function fetchGroupMembers(groupId: string, refresh = false) {
    if (refresh) {
      groupMembers.value = []
      nextMembersCursor.value = null
    }
    
    isLoadingMembers.value = true
    try {
      const cursor = nextMembersCursor.value !== null ? nextMembersCursor.value : undefined;
      const response = await groupService.getGroupMembers({
        group_id: groupId,
        cursor: cursor,
        limit: 20
      })
      
      if (refresh) {
        groupMembers.value = response.items
      } else {
        groupMembers.value = [...groupMembers.value, ...response.items]
      }
      
      hasMoreMembers.value = response.pagination.has_more
      nextMembersCursor.value = (response.pagination.next_cursor as number) || null
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取群组成员失败'
      throw err
    } finally {
      isLoadingMembers.value = false
    }
  }
  
  // Action: 加入群组
  async function joinGroup(groupId: string, password?: string) {
    try {
      const response = await groupService.joinGroup({
        group_id: groupId,
        password
      })
      
      // 刷新用户群组列表
      fetchUserGroups(true)
      
      // 如果已经有群组详情，更新成员状态
      if (currentGroup.value && currentGroup.value.group_id === groupId) {
        currentGroup.value.is_member = true
        currentGroup.value.user_role = response.role
      }
      
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '加入群组失败'
      throw err
    }
  }
  
  // Action: 退出群组
  async function leaveGroup(groupId: string) {
    try {
      const response = await groupService.leaveGroup({
        group_id: groupId
      })
      
      // 从用户群组列表中移除
      userGroups.value = userGroups.value.filter(group => group.group_id !== groupId)
      
      // 如果已经有群组详情，更新成员状态
      if (currentGroup.value && currentGroup.value.group_id === groupId) {
        currentGroup.value.is_member = false
        currentGroup.value.user_role = '';
      }
      
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '退出群组失败'
      throw err
    }
  }
  
  // Action: 获取最近加入的群组
  async function fetchRecentJoinedGroups() {
    try {
      const groups = await groupService.getRecentJoinedGroups()
      recentJoinedGroups.value = groups
      error.value = null
      return groups
    } catch (err: any) {
      error.value = err.message || '获取最近加入群组失败'
      throw err
    }
  }
  
  // 处理群组列表响应
  function handleGroupsResponse(
    response: PaginatedResponse<GroupInfo>,
    targetList: Ref<GroupInfo[]>,
    refresh: boolean
  ) {
    if (refresh) {
      targetList.value = response.items
    } else {
      targetList.value = [...targetList.value, ...response.items]
    }
    
    hasMoreGroups.value = response.pagination.has_more
    nextGroupsCursor.value = response.pagination.next_cursor || null
  }
  
  return {
    userGroups,
    nearbyGroups,
    currentGroup,
    groupMembers,
    recentJoinedGroups,
    isLoading,
    isLoadingMembers,
    isCreating,
    hasMoreGroups,
    hasMoreMembers,
    error,
    getUserGroups,
    getNearbyGroups,
    getCurrentGroup,
    getGroupMembers,
    getRecentJoinedGroups,
    createGroup,
    fetchUserGroups,
    fetchNearbyGroups,
    getGroupDetail,
    fetchGroupMembers,
    joinGroup,
    leaveGroup,
    fetchRecentJoinedGroups
  }
}, {
  persist: true
}) 