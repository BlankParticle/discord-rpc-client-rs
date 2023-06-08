<script lang="ts">
  export let label: string = "";
  export let value: string;
</script>

<div class="flex justify-between my-1 flex-col font-sans">
  <span class="font-600 px-1 text-xl">{label}</span>
  <input
    type="text"
    class="p-2.5 outline-none border-none rounded-2 font-medium text-lg text-text bg-crust/70 selection:bg-mauve/30"
    bind:value
  />
</div>
