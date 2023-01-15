<script lang="ts">
  import type { Media } from "./Media";

  export let media: Media = {
    caption: "",
    media_url: "",
    timestamp: "",
    id: "",
    username: ""
  }
</script>

<div class="item">
    <img src={media.media_url} alt={media.caption} />
    <div>
        <p>{media.username}</p>
        <p>{media.caption}</p>
    </div>
</div>

<style>
.item {
    display: flex;
    align-content: stretch;
    gap: 16px;
}

.item img {
    width: 120px;
}

.item * {
    text-align: start;
}
</style>
