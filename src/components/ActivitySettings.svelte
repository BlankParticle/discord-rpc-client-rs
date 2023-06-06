<script lang="ts">
  export let activity: Activity;
  export let clientId: string;
</script>

<div class="flex flex-col">
  <span class="text-lg font-bold flex justify-center my-2">Settings</span>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Client Id:</span>
    <input type="text" bind:value={clientId} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">State:</span>
    <input type="text" bind:value={activity.state} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Details:</span>
    <input type="text" bind:value={activity.details} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Large Image URL:</span>
    <input type="text" bind:value={activity.assets.large_image} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Large Image Text:</span>
    <input type="text" bind:value={activity.assets.large_text} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Small Image URL:</span>
    <input type="text" bind:value={activity.assets.small_image} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Small Image Text:</span>
    <input type="text" bind:value={activity.assets.small_text} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Timestamp Start:</span>
    <input type="number" bind:value={activity.timestamps.start} />
  </div>
  <div class="flex justify-between my-1">
    <span class="font-semibold mx-2px">Timestamp End:</span>
    <input type="number" bind:value={activity.timestamps.end} />
  </div>
</div>
