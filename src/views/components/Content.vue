<template>
  <div class="w-full h-full box-border p-5">
    <div class="grid grid-cols-5 gap-7">
      <el-card v-for="project in data" :key="project.id" shadow="hover" @click="handleLaunch(project)"
        class="transition-all duration-300 ease-in-out cursor-pointer">
        <template #header>
          <div class="flex justify-between items-center">
            <span class="font-bold text-lg truncate">{{ project.name }}</span>
            
            <el-dropdown @command="(cmd) => handleCommand(cmd, project)" @click.stop>
              <el-icon :size="20" class="cursor-pointer p-1 rounded-full" @click.stop>
                <More />
              </el-icon>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="edit">编辑</el-dropdown-item>
                  <el-dropdown-item command="delete">删除</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>

          </div>
        </template>

        <div class="text-gray-700 text-sm mb-2 py-3 line-clamp-1 truncate">{{ project.projectName }}</div>
        <el-tooltip placement="top" :content="project.location">
          <div class="text-gray-700 text-sm mb-2 py-3 line-clamp-1 truncate">{{ project.location }}</div>
        </el-tooltip>
        <el-tooltip placement="top" :content="project.desc">
          <div class="text-gray-700 text-sm mb-2 py-3 line-clamp-1 truncate">{{ project.desc }}</div>
        </el-tooltip>

        <div class="w-full overflow-x-auto hide-scrollbar">
          <div class="flex flex-nowrap gap-2 mb-4">
            <el-tag v-for="tag in project.tags" :key="tag" size="small">
              {{ tag }}
            </el-tag>
          </div>
        </div>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus';
import { deleteProject, launchProject } from '../../utils/request';
import type { Project } from '../../utils/type';
import { More } from '@element-plus/icons-vue'
import { eventBus } from '../../utils/event';

defineProps<{
  data: Project[]
}>()

async function handleLaunch(item: Project) {
  // 模拟启动项目，避免与菜单操作混淆
  console.log(`Launching project: ${item.name}`);
  await launchProject(item.id!)
}

// 处理下拉菜单命令
const handleCommand = (command: string, project: Project) => {
  if (command === 'edit') {
    handleEdit(project);
  } else if (command === 'delete') {
    handleDelete(project);
  }
}

// 编辑操作的具体实现
const handleEdit = (project: Project) => {
  eventBus.emit('edit-data', project)
}

// 删除操作的具体实现
async function handleDelete(project: Project) {
  await ElMessageBox.confirm('确定要删除?')
  await deleteProject(project.id!)
  ElMessage.success('删除成功')
  eventBus.emit('refresh-page')
}
</script>
