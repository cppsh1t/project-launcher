import { invoke } from "@tauri-apps/api/core";
import type { ListData, Project, ProjectSearchForm } from "./type";

/**
 * 获取项目分页数据
 */
export async function getProjectPage(form: ProjectSearchForm): Promise<ListData<Project>> {
    // 调用 Rust 后端命令 `get_project_page`
    // 参数需要与 Rust 函数签名完全匹配
    return await invoke('get_project_page', {
        page: form.page,
        size: form.size,
        name: form.name,
        tags: form.tags
    });
}

/**
 * 添加一个新项目
 */
export async function addProject(form: Omit<Project, 'id'>): Promise<void> {
    // 调用 Rust 后端命令 `add_project`
    await invoke('add_project', { form });
}

/**
 * 编辑一个现有项目
 * @param form - 必须包含 id
 */
export async function editProject(form: Project): Promise<void> {
    if (form.id === undefined) {
        return Promise.reject(new Error("Project ID is required for editing."));
    }
    // 调用 Rust 后端命令 `edit_project`
    await invoke('edit_project', { form });
}

/**
 * 删除一个项目
 */
export async function deleteProject(id: number): Promise<void> {
    // 调用 Rust 后端命令 `delete_project`
    await invoke('delete_project', { id });
}
