export type ListData<T = any> = {
  list: T[]
  total: number
}


/**
 * @property location 项目地址
 * @property launcher 启动命令， 默认用 code location 启动
 */
export type Project = {
  id?: string
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
  name: string
  tags: string[]
}

export type PageForm<T = any> = {
  page: number
  size: number
  total: number
} & T

export type ProjectSearchForm = PageForm<ProjectSearch>