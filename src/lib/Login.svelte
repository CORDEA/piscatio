<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from "@tauri-apps/api/shell"
  import { Body, getClient, ResponseType } from "@tauri-apps/api/http"

  const AuthorizeUrl = "https://api.instagram.com/"
  const AuthorizePath = "oauth/authorize"
  const AccessTokenPath = "oauth/access_token"
  const RedirectUri = "https://localhost/"

  let opened = false
  let redirectUri = ""

  async function openUrl(){
    const url = new URL(AuthorizeUrl + AuthorizePath)
    const clientId: string = await invoke("client_id")
    url.searchParams.append("client_id", clientId)
    url.searchParams.append("redirect_uri", RedirectUri)
    url.searchParams.append("scope", "user_profile,user_media")
    url.searchParams.append("response_type", "code")
    await open(url.toString())
    opened = true;
  }

  async function login() {
    const url = new URL(redirectUri)
    const code = url.searchParams.get("code")
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
  }
</script>

<div>
  <div class="row">
    {#if opened}
      <input type="text" bind:value={redirectUri} />

      <button on:click={login}>
        Log in
      </button>
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
</style>
