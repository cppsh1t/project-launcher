import mitt from 'mitt'

type EventType = {
    'refresh-page': void
    'add-data': void
}

export const eventBus = mitt<EventType>()