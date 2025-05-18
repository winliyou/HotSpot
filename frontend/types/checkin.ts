// 签到信息
export interface CheckInInfo {
  id: number
  checkin_id: string
  user_id: number
  nickname: string
  description: string
  latitude: number
  longitude: number
  location_name: string
  distance: number
  created_at: string
  tags: string[]
  likes_count: number
  comments_count: number
  liked_by_me: boolean
}

// 创建签到请求
export interface CreateCheckinRequest {
  description: string
  latitude: number
  longitude: number
  location_name: string
  tags?: string[]
}

// 创建签到响应
export interface CreateCheckinResponse {
  checkin_id: string
}

// 删除签到请求
export interface DeleteCheckinRequest {
  checkin_id: string
}

// 删除签到响应
export interface DeleteCheckinResponse {
  success: boolean
}

// 查询签到详情请求
export interface SearchCheckInByIdRequest {
  checkin_id: string
}

// 按位置搜索签到请求
export interface SearchCheckinsByLocationRequest {
  latitude: number
  longitude: number
  radius: number
  cursor?: number
  limit?: number
}

// 查询用户签到历史请求
export interface UserCheckinsHistoryRequest {
  user_id?: number
  cursor?: number
  limit?: number
}

// 按标签搜索签到请求
export interface SearchCheckinsByTagsRequest {
  tags: string[]
  latitude: number
  longitude: number
  cursor?: number
  limit?: number
} 