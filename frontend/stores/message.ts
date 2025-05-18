import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { messageService } from '~/utils/services/message-service'
import type { DirectMessage, Conversation } from '~/types/message'
import type { PaginatedResponse } from '~/utils/api'

export const useMessageStore = defineStore('message', () => {
  // 状态
  const messages = ref<DirectMessage[]>([])
  const conversations = ref<Conversation[]>([])
  const systemNotifications = ref<DirectMessage[]>([])
  const currentConversation = ref<string | null>(null)
  const unreadCounts = ref({
    total: 0,
    system: 0,
    direct: 0,
    checkin: 0,
    like: 0,
    comment: 0
  })
  const isLoading = ref(false)
  const isLoadingMessages = ref(false)
  const isSending = ref(false)
  const hasMoreMessages = ref(false)
  const hasMoreConversations = ref(false)
  const nextMessagesCursor = ref<number | null>(null)
  const nextConversationsCursor = ref<number | null>(null)
  const error = ref<string | null>(null)

  // 计算属性
  const getMessages = computed(() => messages.value)
  const getConversations = computed(() => conversations.value)
  const getSystemNotifications = computed(() => systemNotifications.value)
  const getUnreadCounts = computed(() => unreadCounts.value)
  const getCurrentConversation = computed(() => currentConversation.value)

  // Action: 获取消息会话列表
  async function fetchConversations(refresh = false) {
    if (refresh) {
      conversations.value = []
      nextConversationsCursor.value = null
    }
    
    isLoading.value = true
    try {
      const response = await messageService.getConversations()
      
      if (refresh) {
        conversations.value = response.items
      } else {
        conversations.value = [...conversations.value, ...response.items]
      }
      
      hasMoreConversations.value = response.pagination.has_more
      nextConversationsCursor.value = typeof response.pagination.next_cursor === 'number' ? 
        response.pagination.next_cursor : null
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取会话列表失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 获取私聊消息历史
  async function fetchMessageHistory(conversationId: string, userId?: number, refresh = false) {
    if (refresh) {
      messages.value = []
      nextMessagesCursor.value = null
    }
    
    currentConversation.value = conversationId
    isLoadingMessages.value = true
    
    try {
      const cursor = nextMessagesCursor.value !== null ? nextMessagesCursor.value : undefined;
      const response = await messageService.getDirectMessageHistory({
        conversation_id: conversationId,
        user_id: userId,
        cursor: cursor,
        limit: 20
      })
      
      if (refresh) {
        messages.value = response.items
      } else {
        messages.value = [...messages.value, ...response.items]
      }
      
      hasMoreMessages.value = response.pagination.has_more
      nextMessagesCursor.value = typeof response.pagination.next_cursor === 'number' ? 
        response.pagination.next_cursor : null
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取消息历史失败'
      throw err
    } finally {
      isLoadingMessages.value = false
    }
  }

  // Action: 发送私聊消息
  async function sendDirectMessage(recipientId: number, content: string, location: { latitude: number; longitude: number }) {
    isSending.value = true
    try {
      const response = await messageService.sendDirectMessage({
        recipient_id: recipientId,
        content,
        latitude: location.latitude,
        longitude: location.longitude
      })
      
      // 如果当前正在查看与该用户的会话，刷新消息
      if (currentConversation.value === response.conversation_id) {
        fetchMessageHistory(response.conversation_id, undefined, true)
      }
      
      // 刷新会话列表
      fetchConversations(true)
      
      error.value = null
      return response
    } catch (err: any) {
      error.value = err.message || '发送消息失败'
      throw err
    } finally {
      isSending.value = false
    }
  }

  // Action: 标记消息为已读
  async function markMessageRead(messageId: string, conversationId: string) {
    try {
      await messageService.markMessageRead({ 
        message_id: messageId,
        conversation_id: conversationId
      })
      // 更新本地消息状态
      const message = messages.value.find(m => m.message_id === messageId)
      if (message) {
        message.read_at = new Date().toISOString()
      }
      // 刷新未读计数
      fetchUnreadCount()
      error.value = null
    } catch (err: any) {
      error.value = err.message || '标记消息已读失败'
      throw err
    }
  }

  // Action: 标记会话所有消息为已读
  async function markConversationRead(conversationId: string) {
    try {
      await messageService.markMessageRead({ conversation_id: conversationId })
      // 更新本地状态
      messages.value.forEach(message => {
        if (message.conversation_id === conversationId && !message.read_at) {
          message.read_at = new Date().toISOString()
        }
      })
      // 更新会话未读数
      const conversation = conversations.value.find(c => c.conversation_id === conversationId)
      if (conversation) {
        conversation.unread_count = 0
      }
      // 刷新未读计数
      fetchUnreadCount()
      error.value = null
    } catch (err: any) {
      error.value = err.message || '标记会话已读失败'
      throw err
    }
  }

  // Action: 获取系统通知
  async function fetchSystemNotifications(refresh = false) {
    isLoading.value = true
    try {
      const response = await messageService.getSystemNotifications()
      
      if (refresh) {
        systemNotifications.value = response.items
      } else {
        systemNotifications.value = [...systemNotifications.value, ...response.items]
      }
      
      error.value = null
    } catch (err: any) {
      error.value = err.message || '获取系统通知失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Action: 删除消息
  async function deleteMessage(messageId: string) {
    try {
      await messageService.deleteMessage({ message_id: messageId })
      // 从本地状态中移除该消息
      messages.value = messages.value.filter(m => m.message_id !== messageId)
      error.value = null
    } catch (err: any) {
      error.value = err.message || '删除消息失败'
      throw err
    }
  }

  // Action: 获取未读消息数量
  async function fetchUnreadCount() {
    try {
      const counts = await messageService.getUnreadCount()
      unreadCounts.value = counts
      error.value = null
      return counts
    } catch (err: any) {
      error.value = err.message || '获取未读消息数量失败'
      throw err
    }
  }

  // Action: 加载初始数据
  async function loadInitialData() {
    await Promise.all([
      fetchConversations(true),
      fetchSystemNotifications(true),
      fetchUnreadCount()
    ]);
  }

  return {
    messages,
    conversations,
    systemNotifications,
    unreadCounts,
    currentConversation,
    isLoading,
    isLoadingMessages,
    isSending,
    hasMoreMessages,
    hasMoreConversations,
    error,
    getMessages,
    getConversations,
    getSystemNotifications,
    getUnreadCounts,
    getCurrentConversation,
    fetchConversations,
    fetchMessageHistory,
    sendDirectMessage,
    markMessageRead,
    markConversationRead,
    fetchSystemNotifications,
    deleteMessage,
    fetchUnreadCount,
    loadInitialData
  }
}, {
  persist: true
}) 