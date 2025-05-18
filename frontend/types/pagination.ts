// 通用分页数据返回格式
export interface PaginatedResponse<T> {
  items: T[]
  pagination: {
    has_more: boolean
    next_cursor?: string | number
  }
}

// 分页请求参数
export interface PaginationParams {
  cursor?: string | number
  limit?: number
} 