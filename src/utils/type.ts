export type ListData<T = any> = {
  list: T[]
  total: number
}

/**
 * @property location 项目地址
 * @property launcher 启动命令， 默认用 code location 启动
 */
export type Project = {
  id?: number // 改为 number 以匹配数据库的 i64 类型
  name: string
  projectName: string
  location: string
  tags: string[]
  desc: string
  launcher?: string
};

/**
 * @property name 同时查询name和projectName
 */
export type ProjectSearch = {
  name?: string // 设为可选，以便在不搜索时传递
  tags?: string[] // 设为可选
}

export type PageForm<T = any> = {
  page: number
  size: number
  total: number
} & T

export type ProjectSearchForm = PageForm<ProjectSearch>
