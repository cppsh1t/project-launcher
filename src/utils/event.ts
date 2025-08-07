import mitt from 'mitt'
import type { Project } from './type'

type EventType = {
    'refresh-page': void
    'add-data': void
    'edit-data': Project
}

export const eventBus = mitt<EventType>()