/**
 * API工具类 - 处理前端与后端的HTTP请求
 */

/**
 * API响应结构
 */
export interface ApiResponse<T = any> {
  data?: T
  msg: string
  code: number
}

/**
 * 分页响应接口
 */
export interface PaginatedResponse<T = any> {
  items: T[]
  pagination: {
    has_more: boolean
    next_cursor?: number
  }
}

/**
 * 创建API请求URL
 */
const createApiUrl = (endpoint: string): string => {
  const config = useRuntimeConfig()
  return `${config.public.apiBaseUrl || '/api'}${endpoint}`
}

/**
 * POST请求
 */
export async function apiPost<T = any>(
  endpoint: string,
  body?: Record<string, any> | string
): Promise<T> {
  const url = createApiUrl(endpoint)
  const reqHeaders = useRequestHeaders(['cookie'])

  try {
    const response = await $fetch<ApiResponse<T>>(url, {
      method: 'POST',
      headers: reqHeaders,
      body,
      credentials: 'include',
    })

    if (!response || response.code != 0) {
      throw new Error(response?.msg || '请求失败')
    }
    return response.data as T
  } catch (err: any) {
    throw new Error(err.message || '请求失败')
  }
} 