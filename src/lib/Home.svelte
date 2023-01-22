<script lang="ts">
  import { getClient, ResponseType, Response } from "@tauri-apps/api/http"
  import { onMount, createEventDispatcher } from "svelte";
  import HomeItem from "./HomeItem.svelte";
  import type { MediaResponse } from "./Media";
  import { clearToken, getToken } from "./Store"

  const GraphApiUrl = "https://graph.instagram.com"
  const MediaPath = "/me/media"

  const dispatch = createEventDispatcher()

  let items = []

  onMount(async function () {
    const token = await getToken()
    if (token == null) {
        return
    }

    const client = await getClient()
    const url = new URL(GraphApiUrl + MediaPath)
    url.searchParams.append("access_token", token)
    url.searchParams.append("fields", "id,username,caption,media_url,timestamp")
    const response: Response<MediaResponse> = await client.get(url.toString(), {
        responseType: ResponseType.JSON
    })
    items = response.data.data
  })

  async function logout() {
    await clearToken()
    dispatch("loggedOut")
  }
</script>

<div class="container">
    <div class="header">
        <button on:click={logout}>Logout</button>
    </div>

    {#each items as item}
        <HomeItem media={item} />
    {/each}
</div>

<style>
.container {
    min-width: 600px;
}

.header {
    display: flex;
    justify-content: end;
    margin-bottom: 16px;
}
</style>
