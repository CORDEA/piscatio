<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from "@tauri-apps/api/event"
  import { open } from "@tauri-apps/api/shell"
  import { Body, getClient, ResponseType } from "@tauri-apps/api/http"
  import { onMount } from "svelte";
  import { getToken, putToken } from "./Store"

  const AuthorizeUrl = "https://api.instagram.com/"
  const AuthorizePath = "oauth/authorize"
  const AccessTokenPath = "oauth/access_token"
  const RedirectUri = "https://localhost:8000/login"

  export let loggedIn = false

  let opened = false

  onMount(async function () {
    loggedIn = (await getToken()) != null
  })

  async function openUrl() {
    const url = new URL(AuthorizeUrl + AuthorizePath)
    const clientId: string = await invoke("client_id")
    url.searchParams.append("client_id", clientId)
    url.searchParams.append("redirect_uri", RedirectUri)
    url.searchParams.append("scope", "user_profile,user_media")
    url.searchParams.append("response_type", "code")
    await open(url.toString())
    opened = true;
    await listen("login-code", function (event) {
      const code = event.payload
      if (typeof code === "string") {
        login(code)
      }
    })
  }

  async function login(code: string) {
    const clientId: string = await invoke("client_id")
    const clientSecret: string = await invoke("client_secret")
    const client = await getClient()
    const response = await client.post(
      AuthorizeUrl + AccessTokenPath,
      Body.form({
        client_id: clientId,
        client_secret: clientSecret,
        grant_type: "authorization_code",
        redirect_uri: RedirectUri,
        code: code
      }),
      {
        headers: {"Content-Type": "multipart/form-data"},
        responseType: ResponseType.JSON
      }
    )
    await putToken(response.data["access_token"])
    loggedIn = true
  }
</script>

<div>
  <h1>Piscatio</h1>

  <div class="row">
    {#if opened}
      <h3>Waiting...</h3>
    {:else}
      <button on:click={openUrl}>
        Start
      </button>
    {/if}
  </div>
</div>

<style>
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  outline: none;
}

button:hover {
  border-color: #396cd8;
}

input {
  padding: 8px;
  margin-right: 16px;
}

h1 {
  text-align: center;
  margin-bottom: 36px;
}
</style>
