<script lang="ts">
  import Login from './lib/Login.svelte'
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <Login />
  </div>
</main>

<style>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}
</style>
