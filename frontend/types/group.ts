// 群组信息
export interface GroupInfo {
  group_id: string
  name: string
  description?: string
  creator_id: number
  creator_name: string
  created_at: string
  last_active_at: string
  latitude: number
  longitude: number
  member_count: number
  distance: number
  location_name: string
  is_password_required: boolean
  is_member: boolean
  user_role: string
}

// 创建群组请求
export interface CreateGroupRequest {
  name: string
  description?: string
  password?: string
  latitude: number
  longitude: number
  location_name: string
}

// 创建群组响应
export interface CreateGroupResponse {
  group_id: string
}

// 加入群组请求
export interface JoinGroupRequest {
  group_id: string
  password?: string
}

// 加入群组响应
export interface JoinGroupResponse {
  success: boolean
  role: string
}

// 离开群组请求
export interface LeaveGroupRequest {
  group_id: string
}

// 离开群组响应
export interface LeaveGroupResponse {
  success: boolean
}

// 获取群组成员请求
export interface GroupMembersRequest {
  group_id: string
  cursor?: number
  limit?: number
}

// 群组成员信息
export interface GroupMember {
  user_id: number
  nickname: string
  last_active: string
  role: string
  join_time: string
}

// 获取群组成员响应
export interface GroupMembersResponse {
  members: GroupMember[]
  has_more: boolean
  next_cursor?: number
}

// 按位置搜索群组请求
export interface SearchGroupByLocationRequest {
  latitude: number
  longitude: number
  radius: number
  cursor?: number
  limit?: number
}

// 按名称搜索群组请求
export interface SearchGroupByNameRequest {
  keyword: string
  cursor?: number
  limit?: number
} 