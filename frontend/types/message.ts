// 群组消息
export interface GroupMessage {
  message_id: string
  group_id: string
  sender_id: number
  sender_name: string
  content: string
  message_type: string
  sent_at: string
  latitude: number
  longitude: number
}

// 私聊消息
export interface DirectMessage {
  message_id: string
  conversation_id: string
  sender_id: number
  sender_name: string
  recipient_id: number
  recipient_name: string
  content: string
  message_type: string
  sent_at: string
  read_at: string | null
  latitude: number
  longitude: number
}

// 会话信息
export interface Conversation {
  conversation_id: string
  peer_id: number
  peer_name: string
  last_message: string
  last_message_time: string
  unread_count: number
}

// 发送群组消息请求
export interface SendGroupMessageRequest {
  group_id: string
  content: string
  message_type?: string
  latitude: number
  longitude: number
}

// 发送群组消息响应
export interface SendGroupMessageResponse {
  message_id: string
  group_id: string
  message_type: string
  sent_at: string
}

// 获取群组消息历史请求
export interface GroupMessageHistoryRequest {
  group_id: string
  cursor?: number
  limit?: number
}

// 发送私聊消息请求
export interface SendDirectMessageRequest {
  recipient_id: number
  content: string
  message_type?: string
  latitude: number
  longitude: number
}

// 发送私聊消息响应
export interface SendDirectMessageResponse {
  message_id: string
  conversation_id: string
  recipient_id: number
  message_type: string
  sent_at: string
}

// 获取私聊消息历史请求
export interface DirectMessageHistoryRequest {
  conversation_id?: string
  user_id?: number
  limit?: number
  cursor?: number
}

// 获取会话列表请求
export interface ConversationsRequest {
  limit?: number
  cursor?: number
}

// 标记消息已读请求
export interface MarkReadRequest {
  conversation_id: string
  message_id?: string
}

// 标记消息已读响应
export interface MarkReadResponse {
  success: boolean
  marked_count?: number
}

// 删除私聊消息请求
export interface DeleteDirectMessageRequest {
  message_id: string
}

// 删除消息响应
export interface DeleteMessageResponse {
  success: boolean
}

// 分页响应结构
export interface PaginatedResponse<T = any> {
  items: T[]
  has_more: boolean
  next_cursor?: number | null
} 