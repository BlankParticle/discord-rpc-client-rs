<script lang="ts">
  import { event, invoke } from "@tauri-apps/api";
  // import UserPreview from "./components/UserPreview.svelte";
  import ActivitySettings from "./components/ActivitySettings.svelte";

  let connected = true;
  invoke("try_connecting").then(() => {
    connected = true;
  });

  let activity: Activity = {
    details: "Its working",
    state: "Somehow Happy",
    assets: {
      large_image:
        "https://media.tenor.com/v1fqDQIZbW4AAAAC/konata-izumi-anime.gif",
      large_image_key: "Yay!",
    },
    timestamps: {
      start: Date.now(),
    },
  };
  let clientId = "1113164486161997925";

  const setActivity = () => {
    invoke("handshake", { clientId: clientId }).then(() => {
      invoke("set_activity", { activity }).then(() => {
        alert("Activity Set");
      });
    });
  };
</script>

{#if !connected}
  <h1>Trying to Connect...</h1>
{:else}
  <main class="flex items-center justify-center flex-col">
    <!-- <UserPreview bind:user bind:activity bind:activityName /> -->
    <ActivitySettings bind:activity bind:clientId />
    <div>
      <button
        on:click={setActivity}
        class="my-2 outline-none border-none bg-emerald px-2 py-1 font-bold rounded-1"
      >
        Set Activity
      </button>
    </div>
  </main>
{/if}

<!-- <input type="text" bind:value={message} />
<button on:click={setActivity}>Send</button> -->
