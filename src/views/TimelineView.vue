<template>
    <div class="timeline-view">
        <v-timeline side="end" align="start" class="timeline-force-left"
            line-color=var(--md-sys-color-outline)>
            
            <template v-for="group in timelineGroups" :key="group.id">
                <!-- 时间线组标题 - 从服务获取 -->
                <v-timeline-item :dot-color="group.color" size="large" fill-dot>
                    <template v-slot:icon>
                        <v-avatar :color="group.color">
                            <v-icon color="white">{{ group.iconName }}</v-icon>
                        </v-avatar>
                    </template>
                    <div class="timeline-group-title">{{ group.title }}</div>
                </v-timeline-item>

                <!-- 该组的所有项目 -->
                <v-timeline-item 
                    v-for="item in getItemsByGroup(group.dateGroup)" 
                    :key="item.id" 
                    :dot-color="item.color" 
                    :icon="item.icon"
                    size="small" 
                    density="compact">
                    <EventCard :data="formatCardData(item, group.dateGroup)" @update="(data: EventCardData) => updateItem(data, group.dateGroup)" />
                </v-timeline-item>
            </template>
        </v-timeline>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import EventCard from '@/components/Cards/EventCard.vue'
import timelineService, { TimelineItem, EventCardData } from '@/services/TimelineDataService'

// 使用服务获取时间线组数据
const timelineGroups = computed(() => timelineService.getTimelineGroups());

// 获取特定组的项目
const getItemsByGroup = (dateGroup: string): TimelineItem[] => {
    return timelineService.getItemsByGroup(dateGroup);
};

// 使用服务的格式转换函数，添加明确的类型定义
const formatCardData = (item: TimelineItem, dateGroup: string): EventCardData => {
    return timelineService.formatCardData(item, dateGroup);
};

// 使用服务的更新函数，添加明确的类型定义
const updateItem = (updatedData: EventCardData, dateGroup: string): void => {
    timelineService.updateItem(updatedData, dateGroup);
};
</script>

<style scoped>
.timeline-view {
    width: 100% !important;
    max-width: 900px !important;
    margin: 0 auto !important;
    padding: 16px 16px 0 16px !important;
}

/* 强制时间线左对齐并占满宽度 */
:deep(.timeline-force-left) {
    width: 100% !important;
    margin-left: 0 !important;
    padding-left: 0 !important;
    justify-content: flex-start !important;
}

/* 强制整个时间线组件占满容器宽度 */
:deep(.v-timeline) {
    width: 100% !important;
    max-width: 100% !important;
    padding: 0 !important;
}

/* 时间线内的其他样式 */
.timeline-group-title {
    font-size: 18px;
    font-weight: 600;
    margin: 4px 0;
    color: var(--md-sys-color-on-surface);
}

/* 强制时间线项目占满可用空间 */
:deep(.v-timeline-item) {
    width: 100% !important;
    margin-bottom: -12px;
}

/* 让时间线项目的布局更紧凑 */
:deep(.v-timeline-item) {
    margin-bottom: -12px;
}

/* 让带有组标题的项目有适当的间距 */
:deep(.v-timeline-item--fill-dot) {
    margin-top: 24px;
    margin-bottom: 8px;
}

/* 这是最关键的部分 - 时间线项的内容区域 */
:deep(.v-timeline-item__body) {
    width: calc(100% - 36px) !important;
    /* 减去图标和间距的宽度 */
    max-width: none !important;
    padding-right: 0 !important;
}

/* 给时间线项内的内容增加水平空间 */
:deep(.v-timeline-item__opposite),
:deep(.v-timeline-item__content) {
    width: 100% !important;
    max-width: 100% !important;
    flex: 1 1 auto !important;
}
</style>