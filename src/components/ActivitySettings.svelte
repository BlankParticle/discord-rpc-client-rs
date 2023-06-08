<script lang="ts">
  import TextInput from "./TextInput.svelte";

  export let activity: Activity;
  export let clientId: string;
</script>

<div class="flex flex-col">
  <TextInput label="Client ID" bind:value={clientId} />
  <TextInput label="State" bind:value={activity.state} />
  <TextInput label="Details" bind:value={activity.details} />
  <TextInput label="Large Image URL" bind:value={activity.assets.large_image} />
  <TextInput label="Large Image Text" bind:value={activity.assets.large_text} />
  <TextInput label="Small Image URL" bind:value={activity.assets.small_image} />
  <TextInput label="Small Image Text" bind:value={activity.assets.small_text} />

  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Timestamp Start:</span>
    <input type="number" bind:value={activity.timestamps.start} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Timestamp End:</span>
    <input type="number" bind:value={activity.timestamps.end} />
  </div>
</div>
