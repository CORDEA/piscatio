<script lang="ts">
  import { getClient, ResponseType, Response } from "@tauri-apps/api/http"
  import { onMount } from "svelte";
  import type { MediaResponse } from "./Media";
  import { getToken } from "./Store"

  let GraphApiUrl = "https://graph.instagram.com"
  let MediaPath = "/me/media"

  onMount(async function () {
    const token = await getToken()
    if (token == null) {
        return
    }

    const client = await getClient()
    const url = new URL(GraphApiUrl + MediaPath)
    url.searchParams.append("access_token", token)
    const response: Response<MediaResponse> = await client.get(url.toString(), {
        responseType: ResponseType.JSON
    })
  })
</script>

<div>

</div>

<style>
</style>
