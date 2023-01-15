<script lang="ts">
  import Login from "./lib/Login.svelte"
  import Home from "./lib/Home.svelte"

  let loggedIn = false;
</script>

<main class="container">
  <div class="row">
    {#if loggedIn}
      <Home />
    {:else}
      <Login bind:loggedIn={loggedIn} />
    {/if}
  </div>
</main>

<style>
.container {
  margin: 36px 24px;
}
</style>
