<template>
    <div class="calendar-container">
        <div class="calendar-sidebar">
            <div class="calendar-sidebar-section">
                <h2>操作说明</h2>
                <ul>
                    <li>点击日期可以创建新事件</li>
                    <li>可以拖拽和调整事件</li>
                    <li>点击事件可以删除</li>
                </ul>
            </div>
            <div class="calendar-sidebar-section">
                <label class="weekend-toggle">
                    <input type="checkbox" :checked="calendarOptions.weekends"
                        @change="calendarOptions.weekends = !calendarOptions.weekends" />
                    显示周末
                </label>
            </div>
            <div class="calendar-sidebar-section">
                <h2>所有事件 ({{ currentEvents.length }})</h2>
                <ul class="event-list">
                    <li v-for="event in currentEvents" :key="event.id">
                        <span class="event-time">{{ event.startStr }}</span>
                        <span class="event-title">{{ event.title }}</span>
                    </li>
                </ul>
            </div>
        </div>
        <div class="calendar-main">
            <FullCalendar class="calendar" :options="calendarOptions">
                <template v-slot:eventContent="arg">
                    <div class="event-content">
                        <span class="event-time">{{ arg.timeText }}</span>
                        <span class="event-title">{{ arg.event.title }}</span>
                    </div>
                </template>
            </FullCalendar>
        </div>
    </div>
</template>

<script>
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import timeGridPlugin from '@fullcalendar/timegrid'
import interactionPlugin from '@fullcalendar/interaction'

export default {
    name: 'CalendarView',
    components: {
        FullCalendar
    },
    data() {
        return {
            calendarOptions: {
                plugins: [dayGridPlugin, timeGridPlugin, interactionPlugin],
                headerToolbar: {
                    left: 'prev,next today',
                    center: 'title',
                    right: 'dayGridMonth,timeGridWeek,timeGridDay'
                },
                initialView: 'dayGridMonth',
                editable: true,
                selectable: true,
                selectMirror: true,
                dayMaxEvents: true,
                weekends: true,
                select: this.handleDateSelect,
                eventClick: this.handleEventClick,
                eventsSet: this.handleEvents,
                height: 'auto',
                contentHeight: 'auto', 
                aspectRatio: 2.2,
                locale: 'zh-cn',
                buttonText: {
                    today: '今天',
                    month: '月',
                    week: '周',
                    day: '日'
                }
            },
            currentEvents: []
        }
    },
    methods: {
        handleDateSelect(selectInfo) {
            let title = prompt('请输入事件标题')
            let calendarApi = selectInfo.view.calendar

            calendarApi.unselect()

            if (title) {
                calendarApi.addEvent({
                    id: Date.now().toString(),
                    title,
                    start: selectInfo.startStr,
                    end: selectInfo.endStr,
                    allDay: selectInfo.allDay
                })
            }
        },
        handleEventClick(clickInfo) {
            if (confirm(`确定要删除事件"${clickInfo.event.title}"吗？`)) {
                clickInfo.event.remove()
            }
        },
        handleEvents(events) {
            this.currentEvents = events
        }
    }
}
</script>

<style scoped>
.calendar-container {
    display: flex;
    /* 移除固定高度，允许内容自然流动 */
    min-height: calc(100vh - 64px); /* 改为最小高度 */
    width: 100%;
    font-family: var(--md-sys-typescale-body-large-font);
    font-size: var(--md-sys-typescale-body-large-size);
    color: var(--md-sys-color-on-surface);
    overflow: visible; /* 允许内容溢出 */
    flex-direction: column; /* 在小屏幕上垂直堆叠 */
}

/* 在大屏幕上保持水平布局 */
@media (min-width: 768px) {
    .calendar-container {
        flex-direction: row;
    }
}

.calendar-sidebar {
    width: 100%; /* 在小屏幕上占满宽度 */
    padding: 1rem;
    background: var(--md-sys-color-surface-container);
    border-right: none;
    border-bottom: 1px solid var(--md-sys-color-outline);
    /* 移除滚动属性 */
    overflow: visible;
    height: auto;
}

@media (min-width: 768px) {
    .calendar-sidebar {
        width: 280px; 
        border-right: 1px solid var(--md-sys-color-outline);
        border-bottom: none;
        height: auto;
    }
}

.calendar-sidebar-section {
    margin-bottom: 2rem;
}

.calendar-sidebar-section h2 {
    font-size: var(--md-sys-typescale-title-medium-size);
    margin-bottom: 1rem;
    color: var(--md-sys-color-on-surface);
}

.calendar-sidebar-section ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.calendar-sidebar-section li {
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 4px;
}

.weekend-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
}

.event-list li {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.event-time {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
}

.event-title {
    font-weight: 500;
}

.calendar-main {
    flex: 1;
    padding: 1rem;
    /* 移除滚动属性 */
    overflow: visible;
    height: auto;
}

.calendar {
    height: auto; /* 从 100% 改为 auto，让内容决定高度 */
    min-height: 600px; /* 确保有最小高度 */
    background: var(--md-sys-color-surface);
    border-radius: 8px;
    box-shadow: var(--md-sys-elevation-1);
}

.event-content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.25rem;
}

:deep(.fc) {
    --fc-border-color: var(--md-sys-color-outline);
    --fc-button-bg-color: var(--md-sys-color-primary);
    --fc-button-border-color: var(--md-sys-color-primary);
    --fc-button-text-color: var(--md-sys-color-on-primary);
    --fc-button-active-bg-color: var(--md-sys-color-primary-container);
    --fc-button-active-border-color: var (--md-sys-color-primary-container);
    --fc-button-active-text-color: var(--md-sys-color-on-primary-container);
    --fc-button-hover-bg-color: var(--md-sys-color-primary-container);
    --fc-button-hover-border-color: var(--md-sys-color-primary-container);
    --fc-button-hover-text-color: var(--md-sys-color-on-primary-container);
    --fc-event-bg-color: var(--md-sys-color-primary-container);
    --fc-event-border-color: var(--md-sys-color-primary-container);
    --fc-event-text-color: var(--md-sys-color-on-primary-container);
    --fc-today-bg-color: var(--md-sys-color-surface-container-high);
}
</style>