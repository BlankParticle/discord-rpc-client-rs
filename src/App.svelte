<script lang="ts">
  import { event } from "@tauri-apps/api";
  let message: string = "";
  const send = () => {
    event.emit("set-activity", { state: "It Works", details: message });
  };
</script>

<input type="text" bind:value={message} />
<button on:click={send}>Send</button>
