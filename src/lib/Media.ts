export interface MediaResponse {
    data: Media[]
}

export interface Media {
    caption: string
    id: string
    media_url: string
    timestamp: string
    username: string
}
