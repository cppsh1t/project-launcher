import type { ListData, Project, ProjectSearchForm } from "./type";

export async function getProjectPage(form: ProjectSearchForm): Promise<ListData<Project>> {
    // 实际应用中，这里会根据 form 中的分页和查询条件进行筛选
    // 此处仅为演示，返回所有伪造数据
    const list = [
        {
            name: "后台管理系统",
            projectName: "admin-dashboard-vue3",
            location: "/Users/dev/workspace/company/admin-dashboard-vue3",
            tags: ["Vue3", "TypeScript", "Vite", "Pinia", "Internal"],
            desc: "公司核心业务的后台管理界面，使用最新的Vue技术栈构建。",
            launcher: "pnpm dev"
        },
        {
            name: "个人博客 - '代码农夫'",
            projectName: "my-blog-astro",
            location: "/Users/dev/workspace/personal/my-blog-astro",
            tags: ["Astro", "React", "MDX", "Blog", "OpenSource"],
            desc: "基于Astro和React构建的个人博客，用于记录技术学习和分享生活。"
        },
        {
            name: "Nova UI 组件库",
            projectName: "nova-ui",
            location: "/Users/dev/workspace/side-project/nova-ui",
            tags: ["React", "Storybook", "TypeScript", "Styled-Components", "Library", "React", "Storybook", "TypeScript", "Styled-Components", "Library"],
            desc: "一个轻量级、可定制的React组件库，旨在提升开发效率。",
            launcher: "pnpm storybook"
        },
        {
            name: "购物清单App",
            projectName: "shopping-list-react-native",
            location: "D:\\dev\\mobile\\shopping-list-react-native",
            tags: ["React Native", "Expo", "Mobile", "Personal"],
            desc: "一个跨平台的购物清单应用，使用React Native和Expo开发。",
            launcher: "npx expo start"
        },
        {
            name: "项目API服务",
            projectName: "project-api-nestjs",
            location: "/Users/dev/workspace/backend/project-api-nestjs",
            tags: ["Node.js", "NestJS", "TypeScript", "Backend", "API"],
            desc: "为前端项目提供数据支持的后端API服务。"
        },
        {
            name: "旧版官网维护",
            projectName: "legacy-website-jquery",
            location: "/Users/dev/workspace/archive/legacy-website",
            tags: ["jQuery", "HTML", "CSS", "Legacy"],
            desc: "公司旧版官方网站，目前仅做维护，不再开发新功能。"
        }
    ]
    return Promise.resolve({
        total: 10,
        list
    })
}

export async function addProject(form: Project) {
    return Promise.resolve()
}

export async function editProjecy(form: Project) {
    return Promise.resolve()
}