import { apiPost } from '../api'
import type {
  DirectMessage,
  Conversation,
  SendDirectMessageRequest,
  SendDirectMessageResponse,
  DirectMessageHistoryRequest,
  MarkReadRequest,
  DeleteDirectMessageRequest
} from '~/types/message'
import type { PaginatedResponse } from '../api'

/**
 * 消息服务，提供消息相关功能
 */
export const messageService = {
  /**
   * 发送私聊消息
   * @param data 消息参数
   * @returns 发送结果
   */
  async sendDirectMessage(data: SendDirectMessageRequest): Promise<SendDirectMessageResponse> {
    return await apiPost<SendDirectMessageResponse>('/message/send_direct', data)
  },

  /**
   * 获取私聊消息历史
   * @param data 消息历史参数
   * @returns 消息历史
   */
  async getDirectMessageHistory(data: DirectMessageHistoryRequest): Promise<PaginatedResponse<DirectMessage>> {
    return await apiPost<PaginatedResponse<DirectMessage>>('/message/direct_history', data)
  },

  /**
   * 获取消息会话列表
   * @returns 会话列表
   */
  async getConversations(): Promise<PaginatedResponse<Conversation>> {
    return await apiPost<PaginatedResponse<Conversation>>('/message/conversations')
  },

  /**
   * 标记消息为已读
   * @param data 标记已读参数
   * @returns 成功标志
   */
  async markMessageRead(data: MarkReadRequest): Promise<{success: boolean}> {
    return await apiPost<{success: boolean}>('/message/mark_read', data)
  },

  /**
   * 获取系统通知
   * @returns 系统通知
   */
  async getSystemNotifications(): Promise<PaginatedResponse<DirectMessage>> {
    return await apiPost<PaginatedResponse<DirectMessage>>('/message/notifications')
  },

  /**
   * 删除消息
   * @param data 删除消息参数
   * @returns 成功标志
   */
  async deleteMessage(data: DeleteDirectMessageRequest): Promise<{success: boolean}> {
    return await apiPost<{success: boolean}>('/message/delete', data)
  },

  /**
   * 获取未读消息数量
   * @returns 未读消息数量
   */
  async getUnreadCount(): Promise<{
    total: number,
    system: number,
    direct: number,
    checkin: number,
    like: number,
    comment: number
  }> {
    return await apiPost<{
      total: number,
      system: number,
      direct: number,
      checkin: number,
      like: number,
      comment: number
    }>('/message/unread_count')
  }
} 