<script lang="ts">
  import Login from "./lib/Login.svelte"
  import Home from "./lib/Home.svelte"

  let loggedIn = false;
</script>

<main class="container">
  {#if loggedIn}
    <Home />
  {:else}
    <Login bind:loggedIn={loggedIn} />
  {/if}
</main>

<style>
.container {
  margin: 36px 24px;
}
</style>
