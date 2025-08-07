<template>
  <div class="w-full h-full box-border p-5">
    <div class="grid grid-cols-5 gap-7">
      <el-card v-for="(project, index) in data" :key="index" shadow="hover"
        class="transition-all duration-300 ease-in-out hover:-translate-y-1">
        <template #header>
          <div class="flex justify-between items-center">
            <span class="font-bold text-lg truncate">{{ project.name }}</span>
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
          <div class="flex flex-nowrap  gap-2 mb-4 ">
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
import type { Project } from '../../utils/type';


const props = defineProps<{
  data: Project[]
}>()

</script>