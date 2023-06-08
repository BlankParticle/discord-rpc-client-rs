<script lang="ts">
  import { invoke } from "@tauri-apps/api";
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
      large_text: "Yay!",
      small_image:
        "https://i.gifer.com/origin/dc/dcd102532110a6fea1e33bcdb5d31f9a_w200.gif",
      small_text: "I am the King Now",
    },
    timestamps: {
      start: Date.now(),
      end: 0,
    },
    buttons: [],
  };
  let clientId = "1113164486161997925";

  const setActivity = () => {
    invoke("set_activity", { activity }).then(() => {});
  };
</script>

{#if !connected}
  <h1 class="text-center">Trying to Connect...</h1>
{:else}
  <main class="flex items-center justify-center flex-col">
    <!-- <UserPreview bind:user bind:activity bind:activityName /> -->
    <button
      on:click={() => {
        invoke("handshake", { clientId: clientId });
      }}>handshake</button
    >
    <ActivitySettings bind:activity bind:clientId />
    <div>
      <button
        on:click={setActivity}
        class="m-2 flex px-4.5 py-3 cursor-pointer rounded-50px outline-none border-none bg-teal text-mantle font-sans font-bold active:scale-85 active:bg-teal/80 transition-all-300"
      >
        Set Activity
      </button>
    </div>
  </main>
{/if}
