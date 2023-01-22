import { Store } from "tauri-plugin-store-api"

const TokenKey = "key_token"

const store = new Store(".settings.dat")

export function getToken(): Promise<string | null> {
    return store.get(TokenKey)
}

export async function putToken(token: string) {
    await store.set(TokenKey, token)
}

export async function clearToken() {
    await store.delete(TokenKey)
}
